//! Tenant Middleware

use apexos_core::{
    domain::{TenantId, CorrelationId},
    errors::{ErrorCode, Result, ApexosError},
    http::{AppState, AppState},
    tenancy::{TenantResolver, TenantContext},
};
use axum::{
    extract::{Request, State, Extension},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::{debug, warn, instrument};

// ============================================================================
// Tenant Middleware
// ============================================================================

/// Resolve tenant from request and add to extensions
#[instrument(skip(state, request, next))]
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    // Try to resolve tenant from various sources
    let tenant_id = resolve_tenant(&request, &state.tenant_resolver).await?;
    
    // Get tenant context
    let tenant_context = state.tenant_resolver.get_context(tenant_id).await?;
    
    // Add tenant ID and context to request extensions
    request.extensions_mut().insert(tenant_id);
    request.extensions_mut().insert(tenant_context);
    
    // Add tenant ID to response headers
    let mut response = next.run(request).await;
    
    response.headers_mut().insert(
        "x-tenant-id",
        tenant_id.to_string().parse().unwrap(),
    );
    
    Ok(response)
}

/// Optional tenant middleware - doesn't fail if tenant not found
#[instrument(skip(state, request, next))]
pub async fn optional_tenant_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApexosError> {
    if let Ok(tenant_id) = resolve_tenant(&request, &state.tenant_resolver).await {
        if let Ok(context) = state.tenant_resolver.get_context(tenant_id).await {
            request.extensions_mut().insert(tenant_id);
            request.extensions_mut().insert(context);
        }
    }
    
    Ok(next.run(request).await)
}

/// Resolve tenant from request
async fn resolve_tenant(
    request: &Request,
    resolver: &dyn crate::services::auth::TenantResolverTrait,
) -> Result<crate::domain::TenantId, ApexosError> {
    // 1. Check x-tenant-id header
    if let Some(tenant_header) = request.headers().get("x-tenant-id") {
        if let Ok(tenant_str) = tenant_header.to_str() {
            if let Ok(tenant_id) = crate::domain::TenantId::from_string(tenant_str) {
                return Ok(tenant_id);
            }
        }
    }
    
    // 2. Check x-organization-id header
    if let Some(org_header) = request.headers().get("x-organization-id") {
        if let Ok(org_str) = org_header.to_str() {
            if let Ok(org_id) = crate::domain::OrganizationId::from_string(org_str) {
                // Would resolve tenant from organization
                // For now, return default
            }
        }
    }
    
    // 3. Check Host header for subdomain/domain
    if let Some(host_header) = request.headers().get("host") {
        if let Ok(host_str) = host_header.to_str() {
            // Remove port if present
            let host = host_header.to_str().unwrap_or("").split(':').next().unwrap_or("");
            
            // Try to resolve from subdomain
            if let Ok(tenant_id) = resolve_from_host(host).await {
                return Ok(tenant_id);
            }
        }
    }
    
    // 4. Check JWT claims for tenant_id (would be in auth middleware)
    // This would be set by auth middleware
    
    // 5. Return default tenant
    Ok(crate::domain::TenantId::new())
}

/// Resolve tenant from host header
async fn resolve_from_host(host: &str) -> Result<crate::domain::TenantId, ApexosError> {
    // Would query database to find tenant by domain/subdomain
    // For now, return default
    Ok(crate::domain::TenantId::new())
}

/// Extract tenant context for request
pub fn get_tenant_context(request: &Request) -> Option<&crate::services::auth::TenantContext> {
    request.extensions().get::<crate::services::auth::TenantContext>()
}

/// Get tenant ID from request extensions
pub fn get_tenant_id(request: &Request) -> Option<crate::domain::TenantId> {
    request.extensions().get::<crate::domain::TenantId>().copied()
}

// ============================================================================
// Tenant Extractor for Handlers
// ============================================================================

/// Axum extractor for tenant context
pub struct TenantExtractor {
    pub tenant_id: crate::domain::TenantId,
    pub tenant_context: crate::services::auth::TenantContext,
}

impl axum::extract::FromRequestParts<AppState> for TenantExtractor {
    type Rejection = crate::errors::ApexosError;
    
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Try to get from extensions (set by middleware)
        if let Some(tenant_id) = parts.extensions.get::<crate::domain::TenantId>() {
            if let Some(context) = parts.extensions.get::<crate::services::auth::TenantContext>() {
                return Ok(Self {
                    tenant_id: *tenant_id,
                    tenant_context: context.clone(),
                });
            }
        }
        
        // Fallback: resolve from headers
        let headers = &parts.headers;
        let tenant_id = resolve_tenant_from_headers(headers).await?;
        let context = state.tenant_resolver.get_context(tenant_id).await?;
        
        Ok(Self {
            tenant_id,
            tenant_context: context,
        })
    }
}

async fn resolve_tenant_from_headers(headers: &HeaderMap) -> Result<crate::domain::TenantId, ApexosError> {
    // Check x-tenant-id header
    if let Some(tenant_header) = headers.get("x-tenant-id") {
        if let Ok(tenant_str) = tenant_header.to_str() {
            if let Ok(tenant_id) = crate::domain::TenantId::from_string(tenant_str) {
                return Ok(tenant_id);
            }
        }
    }
    
    // Check x-organization-id header
    if let Some(org_header) = headers.get("x-organization-id") {
        if let Ok(org_str) = org_header.to_str() {
            if let Ok(_org_id) = crate::domain::OrganizationId::from_string(org_str) {
                // Would resolve tenant from organization
            }
        }
    }
    
    // Check Host header
    if let Some(host_header) = headers.get("host") {
        if let Ok(host_str) = host_header.to_str() {
            let host = host_str.split(':').next().unwrap_or("");
            if let Ok(tenant_id) = resolve_from_host(host).await {
                return Ok(tenant_id);
            }
        }
    }
    
    // Return default tenant
    Ok(crate::domain::TenantId::new())
}

use axum::http::HeaderMap;