//! Apexos Auth Service
//!
//! Authentication, Authorization, OAuth2/OIDC, MFA, Session Management
//!
//! Features:
//! - User authentication (email/password, SSO, MFA)
//! - OAuth2/OIDC Provider (Authorization Code, PKCE, Device Code, Client Credentials)
//! - Multi-Factor Authentication (TOTP, WebAuthn, Backup Codes)
//! - Session Management (Redis-backed, rotating tokens)
//! - Password Management (Argon2id, strength validation, history)
//! - Email Verification & Password Reset
//! - Rate Limiting & Brute Force Protection
//! - Audit Logging
//! - gRPC & REST APIs

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::too_many_lines,
    clippy::type_complexity
)]

pub mod config;
pub mod domain;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;
pub mod utils;
pub mod extractors;
pub mod grpc;
pub mod email;

use std::sync::Arc;
use std::time::Duration;

use apexos_core::{
    config::Settings,
    crypto::{TokenGenerator, TokenVerifier},
    crypto::argon2::Argon2Config,
    events::{MessageBus, NatsClient},
    http::{AppState, DatabasePool, CacheClient, ObjectStore},
    observability::{init_tracing, init_metrics, CommonMetrics},
    storage::{create_object_store, create_database_pool, create_cache_client},
    tenancy::{TenantResolver, CachedTenantResolver},
};
use axum::{
    Router,
    routing::{get, post},
    middleware::{self, from_fn_with_state},
};
use sqlx::PgPool;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    request_id::{MakeRequestId, RequestId, PropagateRequestIdLayer, SetRequestIdLayer},
    classify::ServerErrorsAsFailures,
    trace::DefaultMakeSpan,
    LatencyUnit,
};
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load configuration
    let settings = Settings::load()?;
    settings.validate()?;

    // Initialize tracing
    init_tracing(&settings.observability.tracing)?;

    // Initialize metrics
    init_metrics(&settings.observability.metrics)?;

    info!(
        service = %settings.service.name,
        version = %settings.service.version,
        environment = ?settings.service.environment,
        "Starting Apexos Auth Service"
    );

    // Initialize database pool
    let db_pool = create_database_pool(&settings.database).await?;
    let db_pool = Arc::new(db_pool);

    // Run migrations
    run_migrations(&db_pool).await?;

    // Initialize cache client
    let cache_client = create_cache_client(&settings.redis).await?;
    let cache_client = Arc::new(cache_client);

    // Initialize object store
    let object_store = create_object_store(&settings.storage).await?;
    let object_store = Arc::new(object_store);

    // Initialize message bus
    let nats_client = NatsClient::connect(&settings.nats).await?;
    let nats_client = Arc::new(nats_client);
    let message_bus = MessageBus::new().with_nats((*nats_client).clone());

    // Initialize JWT token generator and verifier
    let token_generator = TokenGenerator::new(&settings.auth.jwt)?;
    let token_verifier = TokenVerifier::new(&settings.auth.jwt)?;

    // Initialize tenant resolver
    let tenant_resolver = Arc::new(CachedTenantResolver::new(
        cache_client.clone(),
        Arc::new(PostgresTenantRepo::new(db_pool.clone())),
        settings.service.default_tenant_id.parse()?,
    ));

    // Initialize services
    let auth_service = Arc::new(services::AuthService::new(
        db_pool.clone(),
        cache_client.clone(),
        settings.auth.clone(),
        token_generator,
        token_verifier,
        message_bus.clone(),
        tenant_resolver.clone(),
    ));

    let oauth_service = Arc::new(services::OAuthService::new(
        db_pool.clone(),
        cache_client.clone(),
        settings.auth.oauth2.clone(),
        auth_service.clone(),
        token_generator,
        token_verifier,
        message_bus.clone(),
    ));

    let mfa_service = Arc::new(services::MfaService::new(
        db_pool.clone(),
        cache_client.clone(),
        settings.auth.mfa.clone(),
        message_bus.clone(),
    ));

    let session_service = Arc::new(services::SessionService::new(
        db_pool.clone(),
        cache_client.clone(),
        settings.auth.session.clone(),
        message_bus.clone(),
    ));

    let api_key_service = Arc::new(services::ApiKeyService::new(
        db_pool.clone(),
        cache_client.clone(),
        settings.auth.api_key.clone(),
    ));

    let audit_service = Arc::new(services::AuditService::new(
        db_pool.clone(),
        message_bus.clone(),
    ));

    // Initialize email service
    let email_service = Arc::new(email::EmailService::new(
        settings.integrations.email.clone(),
        db_pool.clone(),
    ));

    // Create metrics
    let metrics = CommonMetrics::new(apexos_core::observability::meter("auth-service"));

    // Build application state
    let app_state = AppState {
        settings: settings.clone(),
        message_bus: message_bus.clone(),
        tenant_resolver,
        token_verifier,
        token_generator,
        metrics: metrics.clone(),
        tracer: apexos_core::observability::tracer("auth-service"),
        cache: cache_client,
        database: db_pool,
        object_store,
    };

    // Build router
    let app = create_router(
        app_state.clone(),
        auth_service,
        oauth_service,
        mfa_service,
        session_service,
        api_key_service,
        audit_service,
        email_service,
    ).await?;

    // Start server
    let addr = settings.server.http.address;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    info!("Auth service listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Auth service shutdown complete");
    Ok(())
}

async fn create_router(
    app_state: AppState,
    auth_service: Arc<services::AuthService>,
    oauth_service: Arc<services::OAuthService>,
    mfa_service: Arc<services::MfaService>,
    session_service: Arc<services::SessionService>,
    api_key_service: Arc<services::ApiKeyService>,
    audit_service: Arc<services::AuditService>,
    email_service: Arc<email::EmailService>,
) -> eyre::Result<Router> {
    let settings = app_state.settings.clone();
    
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(settings.server.http.cors.allowed_origins.iter()
            .map(|o| o.parse().unwrap())
            .collect::<Vec<_>>())
        .allow_methods(settings.server.http.cors.allowed_methods.iter()
            .map(|m| m.parse().unwrap())
            .collect::<Vec<_>>())
        .allow_headers(settings.server.http.cors.allowed_headers.iter()
            .map(|h| h.parse().unwrap())
            .collect::<Vec<_>>())
        .allow_credentials(settings.server.http.cors.allow_credentials)
        .max_age(settings.server.http.cors.max_age);

    // Build router
    let app = Router::new()
        // Health checks
        .route("/health/live", get(handlers::health::liveness))
        .route("/health/ready", get(handlers::health::readiness))
        
        // Public auth endpoints
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/verify-email", post(handlers::auth::verify_email))
        .route("/api/v1/auth/resend-verification", post(handlers::auth::resend_verification))
        .route("/api/v1/auth/forgot-password", post(handlers::auth::forgot_password))
        .route("/api/v1/auth/reset-password", post(handlers::auth::reset_password))
        
        // MFA endpoints
        .route("/api/v1/auth/mfa/enable", post(handlers::mfa::enable_mfa))
        .route("/api/v1/auth/mfa/disable", post(handlers::mfa::disable_mfa))
        .route("/api/v1/auth/mfa/verify", post(handlers::mfa::verify_mfa))
        .route("/api/v1/auth/mfa/backup-codes", post(handlers::mfa::generate_backup_codes))
        
        // Session endpoints
        .route("/api/v1/auth/sessions", get(handlers::sessions::list_sessions))
        .route("/api/v1/auth/sessions/:id", delete(handlers::sessions::revoke_session))
        .route("/api/v1/auth/sessions/all", delete(handlers::sessions::revoke_all_sessions))
        
        // API Key endpoints
        .route("/api/v1/auth/api-keys", post(handlers::api_keys::create_api_key))
        .route("/api/v1/auth/api-keys", get(handlers::api_keys::list_api_keys))
        .route("/api/v1/auth/api-keys/:id", delete(handlers::api_keys::revoke_api_key))
        
        // OAuth2 endpoints
        .route("/oauth/authorize", get(handlers::oauth2::authorize))
        .route("/oauth/token", post(handlers::oauth2::token))
        .route("/oauth/revoke", post(handlers::oauth2::revoke_token))
        .route("/oauth/introspect", post(handlers::oauth2::introspect))
        .route("/oauth/userinfo", get(handlers::oauth2::userinfo))
        .route("/.well-known/jwks.json", get(handlers::oauth2::jwks))
        .route("/.well-known/openid-configuration", get(handlers::oauth2::openid_configuration))
        
        // Protected endpoints (require authentication)
        .route("/api/v1/auth/me", get(handlers::auth::me))
        .route("/api/v1/auth/change-password", post(handlers::auth::change_password))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh_token))
        .route("/api/v1/auth/logout", post(handlers::auth::logout))
        
        // Admin endpoints (require admin role)
        .route("/api/v1/admin/users", get(handlers::admin::list_users))
        .route("/api/v1/admin/users/:id", get(handlers::admin::get_user))
        .route("/api/v1/admin/users", post(handlers::admin::create_user))
        .route("/api/v1/admin/users/:id", put(handlers::admin::update_user))
        .route("/api/v1/admin/users/:id", delete(handlers::admin::delete_user))
        .route("/api/v1/admin/users/:id/suspend", post(handlers::admin::suspend_user))
        .route("/api/v1/admin/users/:id/activate", post(handlers::admin::activate_user))
        .route("/api/v1/admin/audit-logs", get(handlers::admin::audit_logs))
        
        // gRPC reflection endpoint
        .route("/grpc/health", get(handlers::grpc::health))
        
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(RequestIdGenerator::default()))
                .layer(PropagateRequestIdLayer::new(axum::http::HeaderName::from_static("x-request-id")))
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                    .on_failure(tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR))
                    .latency_unit(LatencyUnit::Millis))
                .layer(cors)
                .layer(TimeoutLayer::new(settings.server.http.request_timeout))
                .layer(RequestBodyLimitLayer::new(settings.server.http.body_limit))
                .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                    axum::http::HeaderName::from_static("authorization"),
                )))
                .layer(from_fn_with_state(app_state.clone(), middleware::auth::auth_middleware))
                .layer(from_fn_with_state(app_state.clone(), middleware::tenant::tenant_middleware))
                .layer(from_fn_with_state(app_state.clone(), middleware::rate_limit::rate_limit_middleware))
                .into_inner(),
        )
        .with_state(app_state);

    Ok(app)
}

async fn run_migrations(pool: &PgPool) -> eyre::Result<()> {
    info!("Running database migrations");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("Migrations completed");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down");
        },
    }
}

struct RequestIdGenerator;

impl MakeRequestId for RequestIdGenerator {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let ulid = ulid::Ulid::new();
        Some(RequestId::new(ulid.to_string().parse().unwrap()))
    }
}

// Re-export modules
mod handlers {
    pub mod auth;
    pub mod mfa;
    pub mod sessions;
    pub mod api_keys;
    pub mod oauth2;
    pub mod sessions;
    pub mod admin;
    pub mod health;
    pub mod grpc;
}

mod services {
    pub mod auth;
    pub mod oauth;
    pub mod mfa;
    pub mod session;
    pub mod api_keys;
    pub mod audit;
}

mod middleware {
    pub mod auth;
    pub mod tenant;
    pub mod rate_limit;
}

mod extractors {
    pub mod auth;
    pub mod tenant;
}

// PostgreSQL tenant repository
struct PostgresTenantRepo {
    pool: Arc<PgPool>,
}

impl PostgresTenantRepo {
    fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl apexos_core::tenancy::TenantResolver for PostgresTenantRepo {
    async fn resolve(&self, host: &str) -> apexos_core::errors::Result<apexos_core::domain::TenantId> {
        // Simplified - would query database for matching domain/subdomain
        Ok(apexos_core::domain::TenantId::new())
    }

    async fn get_context(&self, tenant_id: apexos_core::domain::TenantId) -> apexos_core::errors::Result<apexos_core::tenancy::TenantContext> {
        // Simplified - would query database
        Ok(apexos_core::tenancy::TenantContext {
            tenant_id,
            name: "default".to_string(),
            display_name: "Default Tenant".to_string(),
            domain: None,
            subdomain: None,
            status: apexos_core::tenancy::TenantStatus::Active,
            plan: apexos_core::tenancy::TenantPlan::Free,
            settings: Default::default(),
            features: Default::default(),
            organization_ids: Default::default(),
            created_at: apexos_core::domain::Timestamp::now(),
            updated_at: apexos_core::domain::Timestamp::now(),
            deleted_at: None,
        })
    }

    async fn get_by_domain(&self, _domain: &str) -> apexos_core::errors::Result<Option<apexos_core::tenancy::TenantContext>> {
        Ok(None)
    }

    async fn get_by_subdomain(&self, _subdomain: &str) -> apexos_core::errors::Result<Option<apexos_core::tenancy::TenantContext>> {
        Ok(None)
    }

    async fn list(&self, _filter: apexos_core::tenancy::TenantFilter) -> apexos_core::errors::Result<Vec<apexos_core::tenancy::TenantContext>> {
        Ok(vec![])
    }

    async fn create(&self, input: apexos_core::tenancy::CreateTenantInput) -> apexos_core::errors::Result<apexos_core::tenancy::TenantContext> {
        let tenant_id = apexos_core::domain::TenantId::new();
        let now = apexos_core::domain::Timestamp::now();
        
        Ok(apexos_core::tenancy::TenantContext {
            tenant_id,
            name: input.name,
            display_name: input.display_name,
            domain: input.domain,
            subdomain: input.subdomain,
            status: apexos_core::tenancy::TenantStatus::Pending,
            plan: input.plan,
            settings: input.settings.unwrap_or_default(),
            features: Default::default(),
            organization_ids: Default::default(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    async fn update(&self, tenant_id: apexos_core::domain::TenantId, input: apexos_core::tenancy::UpdateTenantInput) -> apexos_core::errors::Result<apexos_core::tenancy::TenantContext> {
        // Simplified
        Ok(apexos_core::tenancy::TenantContext {
            tenant_id,
            name: "default".to_string(),
            display_name: "Default".to_string(),
            domain: None,
            subdomain: None,
            status: apexos_core::tenancy::TenantStatus::Active,
            plan: apexos_core::tenancy::TenantPlan::Free,
            settings: Default::default(),
            features: Default::default(),
            organization_ids: Default::default(),
            created_at: apexos_core::domain::Timestamp::now(),
            updated_at: apexos_core::domain::Timestamp::now(),
            deleted_at: None,
        })
    }

    async fn suspend(&self, _tenant_id: apexos_core::domain::TenantId) -> apexos_core::errors::Result<()> {
        Ok(())
    }

    async fn activate(&self, _tenant_id: apexos_core::domain::TenantId) -> apexos_core::errors::Result<()> {
        Ok(())
    }

    async fn delete(&self, _tenant_id: apexos_core::domain::TenantId) -> apexos_core::errors::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_endpoints() {
        // Test would go here
    }
}