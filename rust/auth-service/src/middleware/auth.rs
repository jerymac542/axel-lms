//! Authentication Middleware

use apexos_core::{
    crypto::TokenVerifier,
    domain::{CorrelationId},
    errors::{ErrorCode, Result, ApexosError},
    http::{AppState, AppState},
    extractors::{AuthUser, OptionalAuthUser},
};
use axum::{
    extract::{Request, State, Extension},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::WithRejection;
use axum_extra::headers::{Authorization, HeaderMapExt};
use axum_extra::TypedHeader;
use std::sync::Arc;
use tracing::{debug, warn, instrument};
use uuid::Uuid;

// ============================================================================
// Authentication Middleware
// ============================================================================

/// Extract and validate JWT token from Authorization header
#[instrument(skip(state, request, next))]
pub async fn auth_middleware(
    State(state): State<AppState>,
    Extension(token_verifier): Extension<Arc<TokenVerifier>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    // Extract Authorization header
    let auth_header = request.headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApexosError::new(ErrorCode::Unauthorized, "Missing Authorization header"))?;
    
    // Parse Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApexosError::new(ErrorCode::Unauthorized, "Invalid Authorization header format"))?;
    
    // Verify token
    let token_data = state.token_verifier.verify(token)
        .map_err(|e| {
            warn!("Token verification failed: {}", e);
            ApexosError::new(ErrorCode::TokenInvalid, "Invalid token")
        })?;
    
    let claims = token_data.claims;
    
    // Check token expiration
    if claims.is_expired() {
        return Err(ApexosError::new(ErrorCode::TokenExpired, "Token has expired"));
    }
    
    // Create AuthUser from claims
    let auth_user = AuthUser {
        claims,
        token: token.to_string(),
    };
    
    // Add auth user to request extensions
    request.extensions_mut().insert(auth_user);
    
    // Add correlation ID if present
    if let Some(correlation_id) = request.headers().get("x-correlation-id") {
        if let Ok(cid) = correlation_id.to_str() {
            request.extensions_mut().insert(CorrelationId::from_string(cid));
        }
    }
    
    Ok(next.run(request).await)
}

/// Optional authentication - doesn't fail if no token present
#[instrument(skip(state, request, next))]
pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    Extension(token_verifier): Extension<Arc<TokenVerifier>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    // Try to extract Authorization header
    if let Some(auth_header) = request.headers().get("authorization").and_then(|h| h.to_str().ok()) {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // Try to verify token
            if let Ok(token_data) = state.token_verifier.verify(token) {
                let claims = token_data.claims;
                
                if !claims.is_expired() {
                    let auth_user = AuthUser {
                        claims,
                        token: token.to_string(),
                    };
                    request.extensions_mut().insert(auth_user);
                }
            }
        }
    }
    
    Ok(next.run(request).await)
}

/// Require specific permission
pub fn require_permission(permission: &'static str) -> impl Fn(Request, Next) -> Result<Response, ApexosError> + Clone + Send + Sync + 'static {
    move |mut request: Request, next: Next| {
        let auth_user = request.extensions()
            .get::<AuthUser>()
            .ok_or_else(|| ApexosError::new(ErrorCode::Unauthorized, "Authentication required"))?;
        
        if !auth_user.claims.has_permission(permission) {
            return Err(ApexosError::new(ErrorCode::Forbidden, format!("Permission required: {}", permission)));
        }
        
        Ok(next.run(request).await?)
    }
}

/// Require specific role
pub fn require_role(role: &'static str) -> impl Fn(Request, Next) -> Result<Response, ApexosError> + Clone + Send + Sync + 'static {
    move |mut request: Request, next: Next| {
        let auth_user = request.extensions()
            .get::<AuthUser>()
            .ok_or_else(|| ApexosError::new(ErrorCode::Unauthorized, "Authentication required"))?;
        
        if !auth_user.claims.has_role(role) {
            return Err(ApexosError::new(ErrorCode::Forbidden, format!("Role required: {}", role)));
        }
        
        Ok(next.run(request).await?)
    }
}

/// Require any of the specified roles
pub fn require_any_role(roles: &'static [&'static str]) -> impl Fn(Request, Next) -> Result<Response, ApexosError> + Clone + Send + Sync + 'static {
    move |mut request: Request, next: Next| {
        let auth_user = request.extensions()
            .get::<AuthUser>()
            .ok_or_else(|| ApexosError::new(ErrorCode::Unauthorized, "Authentication required"))?;
        
        let has_role = roles.iter().any(|r| auth_user.claims.has_role(r));
        
        if !has_role {
            return Err(ApexosError::new(ErrorCode::Forbidden, format!("One of roles required: {:?}", roles)));
        }
        
        Ok(next.run(request).await?)
    }
}

/// Require MFA verification
pub fn require_mfa() -> impl Fn(Request, Next) -> Result<Response, ApexosError> + Clone + Send + Sync + 'static {
    move |mut request: Request, next: Next| {
        let auth_user = request.extensions()
            .get::<AuthUser>()
            .ok_or_else(|| ApexosError::new(ErrorCode::Unauthorized, "Authentication required"))?;
        
        if !auth_user.claims.mfa_verified {
            return Err(ApexosError::new(ErrorCode::MfaRequired, "Multi-factor authentication required"));
        }
        
        Ok(next.run(request).await?)
    }
}

// ============================================================================
// Correlation ID Middleware
// ============================================================================

/// Extract or generate correlation ID
#[instrument(skip(request, next))]
pub async fn correlation_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    let correlation_id = request.headers()
        .get("x-correlation-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| CorrelationId::from_string(s).ok())
        .unwrap_or_else(CorrelationId::new);
    
    request.extensions_mut().insert(correlation_id);
    
    let mut response = next.run(request).await;
    
    // Add correlation ID to response headers
    response.headers_mut().insert(
        "x-correlation-id",
        response.headers().get("x-correlation-id").cloned().unwrap_or_else(|| {
            CorrelationId::new().to_string().parse().unwrap()
        }),
    );
    
    Ok(response)
}

// ============================================================================
// Request ID Middleware
// ============================================================================

/// Generate or propagate request ID
#[instrument(skip(request, next))]
pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    let request_id = request.headers()
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_else(|| {
            let ulid = ulid::Ulid::new();
            ulid.to_string()
        });
    
    // Add to response headers
    let mut response = next.run(request).await;
    
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    Ok(response)
}

// ============================================================================
// Security Headers Middleware
// ============================================================================

/// Add security headers to responses
#[instrument(skip(request, next))]
pub async fn security_headers_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // HSTS
    headers.insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains; preload".parse().unwrap(),
    );
    
    // CSP
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'".parse().unwrap(),
    );
    
    // Referrer Policy
    headers.insert(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    
    // Permissions Policy
    headers.insert(
        "Permissions-Policy",
        "camera=(), microphone=(), geolocation=()".parse().unwrap(),
    );
    
    // X-Frame-Options
    headers.insert(
        "X-Frame-Options",
        "DENY".parse().unwrap(),
    );
    
    // X-Content-Type-Options
    headers.insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap(),
    );
    
    // X-XSS-Protection
    headers.insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );
    
    Ok(response)
}