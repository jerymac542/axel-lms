//! HTTP Layer
//!
//! Axum middleware, extractors, error handling, and API utilities.

use crate::config::{Settings, CorsConfig, TlsConfig};
use crate::crypto::{TokenGenerator, TokenVerifier, JwtClaims};
use crate::domain::{TenantId, UserId, OrganizationId, CorrelationId, SessionId, AggregateId};
use crate::errors::{ApexosError, ErrorCode, ErrorContext, Result};
use crate::events::{EventPublisher, MessageBus};
use crate::observability::{tracer, CommonMetrics};
use crate::tenancy::{TenantContext, TenantResolver};
use crate::validation::ValidationContext;
use axum::{
    body::Body,
    extract::{FromRef, FromRequestParts, Request, State},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode, request::Parts},
    middleware::{self, Next},
    response::{IntoResponse, Response, Json},
    routing::{get, post, put, patch, delete},
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    TypedHeader,
};
use dashmap::DashMap;
use opentelemetry::trace::Tracer;
use opentelemetry::KeyValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower::{ServiceBuilder, Layer};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    request_id::{MakeRequestId, RequestId, PropagateRequestIdLayer, SetRequestIdLayer},
    classify::{ServerErrorsAsFailures, SharedClassifier},
    LatencyUnit,
};
use tracing::{debug, error, info, warn, Span};
use uuid::Uuid;

/// Application state shared across handlers
#[derive(Clone, FromRef)]
pub struct AppState {
    pub settings: Settings,
    pub message_bus: MessageBus,
    pub tenant_resolver: Arc<dyn TenantResolver>,
    pub token_verifier: TokenVerifier,
    pub token_generator: TokenGenerator,
    pub metrics: CommonMetrics,
    pub tracer: opentelemetry::trace::Tracer,
    pub cache: Arc<dyn CacheClient>,
    pub database: Arc<dyn DatabasePool>,
    pub object_store: Arc<dyn ObjectStore>,
}

/// Cache client trait
#[async_trait::async_trait]
pub trait CacheClient: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: &str, value: &[u8], ttl: Duration) -> Result<()>;
    async fn delete(&self, key: &str) -> Result<()>;
    async fn exists(&self, key: &str) -> Result<bool>;
    async fn increment(&self, key: &str, delta: i64) -> Result<i64>;
}

/// Database pool trait
#[async_trait::async_trait]
pub trait DatabasePool: Send + Sync {
    async fn acquire(&self) -> Result<Box<dyn DatabaseConnection>>;
    async fn health_check(&self) -> Result<()>;
}

/// Database connection trait
#[async_trait::async_trait]
pub trait DatabaseConnection: Send + Sync {
    async fn execute(&self, query: &str) -> Result<u64>;
    async fn query(&self, query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>>;
    async fn query_one(&self, query: &str) -> Result<Option<HashMap<String, serde_json::Value>>>;
    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>>;
}

/// Transaction trait
#[async_trait::async_trait]
pub trait Transaction: Send + Sync {
    async fn execute(&self, query: &str) -> Result<u64>;
    async fn query(&self, query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>>;
    async fn commit(self: Box<Self>) -> Result<()>;
    async fn rollback(self: Box<Self>) -> Result<()>;
}

/// Object store trait
#[async_trait::async_trait]
pub trait ObjectStore: Send + Sync {
    async fn put(&self, key: &str, data: &[u8], content_type: &str) -> Result<String>;
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn delete(&self, key: &str) -> Result<()>;
    async fn exists(&self, key: &str) -> Result<bool>;
    async fn presign_get(&self, key: &str, ttl: Duration) -> Result<String>;
    async fn presign_put(&self, key: &str, ttl: Duration, content_type: &str) -> Result<String>;
}

/// Request ID generator
#[derive(Clone, Default)]
pub struct RequestIdGenerator;

impl MakeRequestId for RequestIdGenerator {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let ulid = ulid::Ulid::new();
        Some(RequestId::new(ulid.to_string().parse().unwrap()))
    }
}

/// Extractors
pub mod extractors {
    use super::*;

    /// Extract authenticated user from JWT
    #[derive(Debug, Clone)]
    pub struct AuthUser {
        pub claims: JwtClaims,
        pub token: String,
    }

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for AuthUser
    where
        S: Send + Sync,
        AppState: FromRef<S>,
    {
        type Rejection = ApexosError;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
            let state = AppState::from_ref(state);
            
            // Extract Authorization header
            let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApexosError::new(ErrorCode::Unauthorized, "Missing or invalid Authorization header"))?;

            let token = bearer.token().to_string();
            
            // Verify token
            let token_data = state.token_verifier.verify(&token)?;
            let claims = token_data.claims;

            // Check if token is expired
            if claims.is_expired() {
                return Err(ApexosError::new(ErrorCode::TokenExpired, "Token has expired"));
            }

            Ok(Self { claims, token })
        }
    }

    /// Optional authenticated user (doesn't reject if missing)
    #[derive(Debug, Clone)]
    pub struct OptionalAuthUser(pub Option<AuthUser>);

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for OptionalAuthUser
    where
        S: Send + Sync,
        AppState: FromRef<S>,
    {
        type Rejection = Infallible;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> std::result::Result<Self, Infallible> {
            let state = AppState::from_ref(state);
            
            // Try to extract Authorization header
            let token = if let Ok(TypedHeader(Authorization(bearer))) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
                bearer.token().to_string()
            } else {
                return Ok(Self(None));
            };

            // Try to verify token
            match state.token_verifier.verify(&token) {
                Ok(token_data) => {
                    let claims = token_data.claims;
                    if claims.is_expired() {
                        Ok(Self(None))
                    } else {
                        Ok(Self(Some(AuthUser { claims, token })))
                    }
                }
                Err(_) => Ok(Self(None)),
            }
        }
    }

    /// Extract tenant context
    #[derive(Debug, Clone)]
    pub struct TenantExtractor {
        pub tenant_id: TenantId,
        pub tenant_context: TenantContext,
    }

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for TenantExtractor
    where
        S: Send + Sync,
        AppState: FromRef<S>,
    {
        type Rejection = ApexosError;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
            let state = AppState::from_ref(state);
            
            // First try to get tenant from headers (for API calls)
            let tenant_header = parts.headers.get("x-tenant-id")
                .or_else(|| parts.headers.get("x-organization-id"))
                .and_then(|v| v.to_str().ok());

            let tenant_id = if let Some(header) = tenant_header {
                TenantId::from_str(header).map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid tenant ID"))?
            } else {
                // Resolve from request context (subdomain, domain, etc.)
                let host = parts.headers.get(header::HOST)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");
                
                state.tenant_resolver.resolve(host).await?
            };

            let tenant_context = state.tenant_resolver.get_context(tenant_id).await?;

            Ok(Self { tenant_id, tenant_context })
        }
    }

    /// Optional tenant extractor
    #[derive(Debug, Clone)]
    pub struct OptionalTenantExtractor(pub Option<TenantExtractor>);

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for OptionalTenantExtractor
    where
        S: Send + Sync,
        AppState: FromRef<S>,
    {
        type Rejection = Infallible;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> std::result::Result<Self, Infallible> {
            match TenantExtractor::from_request_parts(parts, state).await {
                Ok(extractor) => Ok(Self(Some(extractor))),
                Err(_) => Ok(Self(None)),
            }
        }
    }

    /// Extract organization context
    #[derive(Debug, Clone)]
    pub struct OrganizationExtractor {
        pub organization_id: OrganizationId,
    }

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for OrganizationExtractor
    where
        S: Send + Sync,
        AppState: FromRef<S>,
    {
        type Rejection = ApexosError;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
            let state = AppState::from_ref(state);
            
            // Try header first
            if let Some(header) = parts.headers.get("x-organization-id").and_then(|v| v.to_str().ok()) {
                let organization_id = OrganizationId::from_str(header).map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid organization ID"))?;
                return Ok(Self { organization_id });
            }

            // Try from authenticated user's claims
            if let Ok(auth) = AuthUser::from_request_parts(parts, state).await {
                if let Some(org_id) = auth.claims.organization_id {
                    let organization_id = OrganizationId::from_str(&org_id).map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid organization ID"))?;
                    return Ok(Self { organization_id });
                }
            }

            Err(ApexosError::new(ErrorCode::OrganizationAccessDenied, "Organization context required"))
        }
    }

    /// Extract correlation ID
    #[derive(Debug, Clone)]
    pub struct CorrelationExtractor {
        pub correlation_id: CorrelationId,
    }

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for CorrelationExtractor
    where
        S: Send + Sync,
    {
        type Rejection = Infallible;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Infallible> {
            let correlation_id = parts.headers.get("x-correlation-id")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| CorrelationId::from_str(v).ok())
                .unwrap_or_else(CorrelationId::new);

            Ok(Self { correlation_id })
        }
    }

    /// Extract request ID
    #[derive(Debug, Clone)]
    pub struct RequestIdExtractor {
        pub request_id: String,
    }

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for RequestIdExtractor
    where
        S: Send + Sync,
    {
        type Rejection = Infallible;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Infallible> {
            let request_id = parts.extensions.get::<RequestId>()
                .map(|id| id.to_string())
                .unwrap_or_else(|| Uuid::now_v7().to_string());

            Ok(Self { request_id })
        }
    }

    /// Combined authentication context
    #[derive(Debug, Clone)]
    pub struct AuthContext {
        pub user: AuthUser,
        pub tenant: TenantExtractor,
        pub organization: Option<OrganizationExtractor>,
        pub correlation_id: CorrelationId,
        pub request_id: String,
    }

    impl AuthContext {
        pub fn user_id(&self) -> UserId {
            UserId::from_str(&self.user.claims.sub).unwrap()
        }

        pub fn tenant_id(&self) -> TenantId {
            self.tenant.tenant_id
        }

        pub fn organization_id(&self) -> Option<OrganizationId> {
            self.organization.as_ref().map(|o| o.organization_id)
        }

        pub fn has_permission(&self, permission: &str) -> bool {
            self.user.claims.has_permission(permission)
        }

        pub fn has_role(&self, role: &str) -> bool {
            self.user.claims.has_role(role)
        }

        pub fn has_scope(&self, scope: &str) -> bool {
            self.user.claims.has_scope(scope)
        }

        pub fn is_mfa_verified(&self) -> bool {
            self.user.claims.mfa_verified
        }

        pub fn session_id(&self) -> Option<SessionId> {
            self.user.claims.session_id.as_ref().and_then(|s| SessionId::from_str(s).ok())
        }
    }

    #[async_trait::async_trait]
    impl<S> FromRequestParts<S> for AuthContext
    where
        S: Send + Sync,
        AppState: FromRef<S>,
    {
        type Rejection = ApexosError;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
            let user = AuthUser::from_request_parts(parts, state).await?;
            let tenant = TenantExtractor::from_request_parts(parts, state).await?;
            let correlation = CorrelationExtractor::from_request_parts(parts, state).await?;
            let request_id = RequestIdExtractor::from_request_parts(parts, state).await?;
            
            let organization = OrganizationExtractor::from_request_parts(parts, state).await.ok();

            // Verify tenant access
            if let Some(org_id) = user.claims.organization_id.as_ref() {
                if let Ok(org) = OrganizationId::from_str(org_id) {
                    if tenant.tenant_context.organization_ids.contains(&org) {
                        // OK
                    } else {
                        return Err(ApexosError::new(ErrorCode::TenantAccessDenied, "User not member of this tenant"));
                    }
                }
            }

            Ok(Self {
                user,
                tenant,
                organization,
                correlation_id: correlation.correlation_id,
                request_id: request_id.request_id,
            })
        }
    }
}

/// Middleware
pub mod middleware {
    use super::*;

    /// Add request/response logging
    pub async fn logging_middleware(
        request: Request<Body>,
        next: Next,
    ) -> Response {
        let method = request.method().clone();
        let uri = request.uri().clone();
        let start = std::time::Instant::now();

        let response = next.run(request).await;

        let latency = start.elapsed();
        let status = response.status();

        info!(
            method = %method,
            uri = %uri,
            status = %status,
            latency_ms = latency.as_millis(),
            "HTTP request"
        );

        response
    }

    /// Add tenant context to request extensions
    pub async fn tenant_middleware(
        State(state): State<AppState>,
        mut request: Request<Body>,
        next: Next,
    ) -> Response {
        // Try to resolve tenant from host header
        if let Some(host) = request.headers().get(header::HOST).and_then(|v| v.to_str().ok()) {
            if let Ok(tenant_id) = state.tenant_resolver.resolve(host).await {
                request.extensions_mut().insert(tenant_id);
                
                if let Ok(context) = state.tenant_resolver.get_context(tenant_id).await {
                    request.extensions_mut().insert(context);
                }
            }
        }

        next.run(request).await
    }

    /// Rate limiting middleware
    pub async fn rate_limit_middleware(
        State(state): State<AppState>,
        request: Request<Body>,
        next: Next,
    ) -> Response {
        // Extract identifier for rate limiting
        let identifier = extract_rate_limit_identifier(&request);
        
        // Check rate limit
        let key = format!("ratelimit:{}", identifier);
        let current = state.cache.increment(&key, 1).await.unwrap_or(0);
        
        let limit = state.settings.security.rate_limiting.default.max_requests as i64;
        let window = state.settings.security.rate_limiting.default.window;

        if current > limit {
            // Set rate limit headers
            let mut response = ApexosError::new(ErrorCode::RateLimited, "Rate limit exceeded")
                .with_field("retry_after", window.as_secs().to_string())
                .into_response();
            
            response.headers_mut().insert(
                header::RETRY_AFTER,
                HeaderValue::from_str(&window.as_secs().to_string()).unwrap(),
            );
            
            return response;
        }

        // Set rate limit headers
        let mut response = next.run(request).await;
        response.headers_mut().insert(
            "X-RateLimit-Limit",
            HeaderValue::from_str(&limit.to_string()).unwrap(),
        );
        response.headers_mut().insert(
            "X-RateLimit-Remaining",
            HeaderValue::from_str(&(limit - current).max(0).to_string()).unwrap(),
        );
        response.headers_mut().insert(
            "X-RateLimit-Reset",
            HeaderValue::from_str(&(chrono::Utc::now() + window).timestamp().to_string()).unwrap(),
        );

        response
    }

    fn extract_rate_limit_identifier(request: &Request<Body>) -> String {
        // Try API key first
        if let Some(api_key) = request.headers().get("x-api-key").and_then(|v| v.to_str().ok()) {
            return format!("apikey:{}", api_key);
        }

        // Try user ID from auth
        if let Some(auth) = request.extensions().get::<extractors::AuthUser>() {
            return format!("user:{}", auth.claims.sub);
        }

        // Fall back to IP
        if let Some(forwarded) = request.headers().get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
            return format!("ip:{}", forwarded.split(',').next().unwrap_or("unknown").trim());
        }

        if let Some(real_ip) = request.headers().get("x-real-ip").and_then(|v| v.to_str().ok()) {
            return format!("ip:{}", real_ip);
        }

        "ip:unknown".to_string()
    }

    /// CORS middleware
    pub fn cors_layer(config: &CorsConfig) -> CorsLayer {
        let mut layer = CorsLayer::new()
            .allow_methods(config.allowed_methods.iter().map(|m| m.parse().unwrap()).collect::<Vec<_>>())
            .allow_headers(config.allowed_headers.iter().map(|h| h.parse().unwrap()).collect::<Vec<_>>())
            .allow_credentials(config.allow_credentials)
            .max_age(config.max_age);

        if config.allowed_origins.contains(&"*".to_string()) {
            layer = layer.allow_origin(Any);
        } else {
            let origins: Vec<HeaderValue> = config.allowed_origins.iter()
                .filter_map(|o| o.parse().ok())
                .collect();
            layer = layer.allow_origin(origins);
        }

        if !config.exposed_headers.is_empty() {
            layer = layer.expose_headers(config.exposed_headers.iter().map(|h| h.parse().unwrap()).collect::<Vec<_>>());
        }

        layer
    }

    /// Security headers middleware
    pub async fn security_headers_middleware(
        request: Request<Body>,
        next: Next,
    ) -> Response {
        let mut response = next.run(request).await;
        
        let headers = response.headers_mut();
        
        // HSTS
        headers.insert(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        );
        
        // X-Frame-Options
        headers.insert(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        );
        
        // X-Content-Type-Options
        headers.insert(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        );
        
        // X-XSS-Protection
        headers.insert(
            "X-XSS-Protection",
            HeaderValue::from_static("1; mode=block"),
        );
        
        // Referrer-Policy
        headers.insert(
            "Referrer-Policy",
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        );
        
        // Permissions-Policy
        headers.insert(
            "Permissions-Policy",
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        );

        response
    }

    /// Request validation middleware
    pub async fn validation_middleware<B>(
        request: Request<B>,
        next: Next,
    ) -> Result<Response, ApexosError>
    where
        B: Send + 'static,
    {
        // Content-Type validation for mutating requests
        if matches!(request.method(), Method::POST | Method::PUT | Method::PATCH) {
            let content_type = request.headers().get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            if !content_type.starts_with("application/json") 
                && !content_type.starts_with("multipart/form-data")
                && !content_type.starts_with("application/x-www-form-urlencoded") {
                return Err(ApexosError::new(ErrorCode::UnsupportedMediaType, "Unsupported Content-Type"));
            }
        }

        Ok(next.run(request).await)
    }

    /// Request size limiting
    pub fn body_limit_layer(limit: usize) -> RequestBodyLimitLayer {
        RequestBodyLimitLayer::new(limit)
    }

    /// Timeout middleware
    pub fn timeout_layer(timeout: Duration) -> TimeoutLayer {
        TimeoutLayer::new(timeout)
    }

    /// Compression middleware
    pub fn compression_layer() -> CompressionLayer {
        CompressionLayer::new()
    }
}

/// Router builder with common middleware
pub struct ApiRouter {
    router: Router<AppState>,
    state: AppState,
}

impl ApiRouter {
    pub fn new(state: AppState) -> Self {
        let cors = middleware::cors_layer(&state.settings.server.http.cors);
        
        let router = Router::new()
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(RequestIdGenerator::default()))
                    .layer(PropagateRequestIdLayer::new(header::HeaderName::from_static("x-request-id")))
                    .layer(TraceLayer::new_for_http()
                        .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
                        .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                        .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                        .on_failure(tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR)))
                    .layer(cors)
                    .layer(middleware::compression_layer())
                    .layer(middleware::timeout_layer(state.settings.server.http.request_timeout))
                    .layer(middleware::body_limit_layer(state.settings.server.http.body_limit))
                    .layer(middleware::security_headers_middleware)
                    .layer(middleware::validation_middleware)
                    .layer(middleware::tenant_middleware)
                    .layer(middleware::rate_limit_middleware)
                    .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                        header::AUTHORIZATION,
                    )))
                    .into_inner(),
            )
            .with_state(state.clone());

        Self { router, state }
    }

    pub fn route(mut self, method: Method, path: &str, handler: impl axum::handler::Handler<T, AppState> + Clone + Send + Sync + 'static, T: 'static) -> Self {
        self.router = match method {
            Method::GET => self.router.route(path, get(handler)),
            Method::POST => self.router.route(path, post(handler)),
            Method::PUT => self.router.route(path, put(handler)),
            Method::PATCH => self.router.route(path, patch(handler)),
            Method::DELETE => self.router.route(path, delete(handler)),
            _ => self.router.route(path, axum::routing::any(handler)),
        };
        self
    }

    pub fn nest(mut self, path: &str, router: Router<AppState>) -> Self {
        self.router = self.router.nest(path, router);
        self
    }

    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<Router<AppState>> + Clone + Send + Sync + 'static,
        L::Service: Send + Sync + 'static,
    {
        self.router = self.router.layer(layer);
        self
    }

    pub fn build(self) -> Router<AppState> {
        self.router
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}

/// Health check endpoints
pub mod health {
    use super::*;
    use axum::Json;

    #[derive(Serialize)]
    pub struct HealthResponse {
        pub status: String,
        pub version: String,
        pub timestamp: String,
        pub checks: HashMap<String, HealthCheck>,
    }

    #[derive(Serialize)]
    pub struct HealthCheck {
        pub status: String,
        pub message: Option<String>,
    }

    pub async fn liveness() -> Json<HealthResponse> {
        Json(HealthResponse {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            checks: HashMap::new(),
        })
    }

    pub async fn readiness(State(state): State<AppState>) -> Result<Json<HealthResponse>> {
        let mut checks = HashMap::new();
        let mut all_healthy = true;

        // Database check
        match state.database.health_check().await {
            Ok(_) => checks.insert("database".to_string(), HealthCheck { status: "ok".to_string(), message: None }),
            Err(e) => {
                all_healthy = false;
                checks.insert("database".to_string(), HealthCheck { status: "fail".to_string(), message: Some(e.to_string()) });
            }
        }

        // Cache check
        match state.cache.get("health-check").await {
            Ok(_) => checks.insert("cache".to_string(), HealthCheck { status: "ok".to_string(), message: None }),
            Err(e) => {
                all_healthy = false;
                checks.insert("cache".to_string(), HealthCheck { status: "fail".to_string(), message: Some(e.to_string()) });
            }
        }

        // Message bus check (NATS)
        // Would need to implement ping

        let status = if all_healthy { "ok" } else { "degraded" }.to_string();

        Ok(Json(HealthResponse {
            status,
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            checks,
        }))
    }

    pub async fn startup() -> Json<HealthResponse> {
        Json(HealthResponse {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            checks: HashMap::new(),
        })
    }
}

/// API versioning
pub mod versioning {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ApiVersion {
        V1,
        V2,
    }

    impl ApiVersion {
        pub fn from_header(headers: &HeaderMap) -> Option<Self> {
            headers.get("api-version")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| match v {
                    "1" | "v1" => Some(Self::V1),
                    "2" | "v2" => Some(Self::V2),
                    _ => None,
                })
        }

        pub fn prefix(&self) -> &'static str {
            match self {
                Self::V1 => "/api/v1",
                Self::V2 => "/api/v2",
            }
        }
    }

    pub fn versioned_router(version: ApiVersion, router: Router<AppState>) -> Router<AppState> {
        Router::new().nest(version.prefix(), router)
    }
}

/// Pagination helpers
pub mod pagination {
    use super::*;
    use crate::domain::{Pagination, Paginated};

    pub fn extract_pagination(headers: &HeaderMap) -> Pagination {
        let page = headers.get("x-page")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let per_page = headers.get("x-per-page")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(50);

        Pagination::new(page, per_page)
    }

    pub fn add_pagination_headers<T>(response: &mut Response, paginated: &Paginated<T>) {
        let headers = response.headers_mut();
        headers.insert("X-Total-Count", HeaderValue::from_str(&paginated.pagination.total_items.to_string()).unwrap());
        headers.insert("X-Total-Pages", HeaderValue::from_str(&paginated.pagination.total_pages.to_string()).unwrap());
        headers.insert("X-Current-Page", HeaderValue::from_str(&paginated.pagination.current_page.to_string()).unwrap());
        headers.insert("X-Per-Page", HeaderValue::from_str(&paginated.pagination.per_page.to_string()).unwrap());
    }
}

/// API response helpers
pub mod response {
    use super::*;

    #[derive(Serialize)]
    pub struct ApiResponse<T> {
        pub data: T,
        pub meta: Option<ResponseMeta>,
    }

    #[derive(Serialize)]
    pub struct ResponseMeta {
        pub timestamp: String,
        pub request_id: String,
        pub version: String,
    }

    impl<T> ApiResponse<T> {
        pub fn new(data: T, request_id: &str) -> Self {
            Self {
                data,
                meta: Some(ResponseMeta {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    request_id: request_id.to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                }),
            }
        }

        pub fn without_meta(data: T) -> Self {
            Self { data, meta: None }
        }
    }

    pub fn created<T: Serialize>(data: T, request_id: &str) -> (StatusCode, Json<ApiResponse<T>>) {
        (StatusCode::CREATED, Json(ApiResponse::new(data, request_id)))
    }

    pub fn ok<T: Serialize>(data: T, request_id: &str) -> Json<ApiResponse<T>> {
        Json(ApiResponse::new(data, request_id))
    }

    pub fn no_content() -> StatusCode {
        StatusCode::NO_CONTENT
    }

    pub fn accepted(request_id: &str) -> Json<ApiResponse<()>> {
        Json(ApiResponse::new((), request_id))
    }
}

/// Error handling
pub mod error_handling {
    use super::*;

    pub async fn handle_error(err: ApexosError) -> Response {
        err.into_response()
    }

    pub async fn handle_any_error(err: Box<dyn std::error::Error + Send + Sync>) -> Response {
        let apexos_err = ApexosError::new(ErrorCode::InternalError, err.to_string()).with_source(err);
        apexos_err.into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_parsing() {
        let mut headers = HeaderMap::new();
        headers.insert("api-version", "v1".parse().unwrap());
        assert_eq!(versioning::ApiVersion::from_header(&headers), Some(versioning::ApiVersion::V1));

        headers.insert("api-version", "2".parse().unwrap());
        assert_eq!(versioning::ApiVersion::from_header(&headers), Some(versioning::ApiVersion::V2));
    }

    #[test]
    fn pagination_extraction() {
        let mut headers = HeaderMap::new();
        headers.insert("x-page", "2".parse().unwrap());
        headers.insert("x-per-page", "25".parse().unwrap());
        
        let pagination = pagination::extract_pagination(&headers);
        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.per_page, 25);
        assert_eq!(pagination.offset(), 25);
    }
}