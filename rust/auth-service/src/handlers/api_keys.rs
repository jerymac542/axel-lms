//! API Keys Handlers

use crate::domain::ApiKey;
use crate::services::api_keys::ApiKeyService;
use apexos_core::{
    domain::{UserId, CorrelationId},
    errors::{ErrorCode, Result, ApexosError},
    extractors::{AuthUser, CorrelationId},
    http::ApiResponse,
};
use axum::{
    extract::{State, Path, Extension},
    response::Json,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, instrument};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub scopes: Option<Vec<String>>,
    pub expires_at: Option<String>, // ISO 8601 datetime
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CreateApiKeyResponse {
    pub api_key: String, // Only shown once
    pub key: ApiKeyResponse,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
    pub revoked: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ApiKeysListResponse {
    pub keys: Vec<ApiKeyResponse>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ApiKeysQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// ============================================================================
// API Keys Handlers
// ============================================================================

/// Create a new API key
#[utoipa::path(
    post,
    path = "/api/v1/auth/api-keys",
    request_body = CreateApiKeyRequest,
    responses(
        (status = 201, description = "API key created", body = CreateApiKeyResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Maximum API keys reached")
    ),
    tag = "api-keys",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, api_key_service))]
pub async fn create_api_key(
    State(state): State<AppState>,
    Extension(api_key_service): Extension<Arc<crate::services::api_keys::ApiKeyService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<CreateApiKeyRequest>, _>,
) -> Result<Json<ApiResponse<CreateApiKeyResponse>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    let tenant_id = crate::domain::TenantId::new(); // Would come from tenant context
    
    let expires_at = payload.expires_at.as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    
    let (api_key, key) = crate::services::api_keys::ApiKeyService::create(
        &api_key_service,
        user_id,
        tenant_id,
        payload.name,
        payload.scopes.unwrap_or_default(),
        expires_at,
    ).await?;
    
    let response = CreateApiKeyResponse {
        api_key: api_key,
        key: ApiKeyResponse {
            id: key.id.to_string(),
            name: key.name,
            prefix: key.prefix,
            scopes: key.scopes,
            created_at: key.created_at.to_string(),
            expires_at: key.expires_at.map(|t| t.to_string()),
            last_used_at: key.last_used_at.map(|t| t.to_string()),
            revoked: key.revoked,
        },
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// List API keys for current user
#[utoipa::path(
    get,
    path = "/api/v1/auth/api-keys",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of API keys", body = ApiKeysListResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "api-keys",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, api_key_service))]
pub async fn list_api_keys(
    State(state): State<AppState>,
    Extension(api_key_service): Extension<Arc<crate::services::api_keys::ApiKeyService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Query(query): Query<ApiKeysQuery>,
) -> Result<Json<ApiResponse<ApiKeysListResponse>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    let keys = crate::services::api_keys::ApiKeyService::list_by_user(
        &api_key_service,
        user_id,
    ).await?;
    
    let key_responses: Vec<ApiKeyResponse> = keys.into_iter().map(|k| ApiKeyResponse {
        id: k.id.to_string(),
        name: k.name,
        prefix: k.prefix,
        scopes: k.scopes,
        created_at: k.created_at.to_string(),
        expires_at: k.expires_at.map(|t| t.to_string()),
        last_used_at: k.last_used_at.map(|t| t.to_string()),
        revoked: k.revoked,
    }).collect();
    
    let response = ApiKeysListResponse {
        keys: key_responses,
        total: 0,
        page: 1,
        per_page: 20,
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// Revoke an API key
#[utoipa::path(
    delete,
    path = "/api/v1/auth/api-keys/{id}",
    params(
        ("id" = String, Path, description = "API Key ID")
    ),
    responses(
        (status = 200, description = "API key revoked"),
        (status = 404, description = "API key not found")
    ),
    tag = "api-keys",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, api_key_service))]
pub async fn revoke_api_key(
    State(state): State<AppState>,
    Extension(api_key_service): Extension<Arc<crate::services::api_keys::ApiKeyService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    crate::services::api_keys::ApiKeyService::revoke(&api_key_service, &id, user_id).await?;
    
    Ok(Json(ApiResponse::new((), "request-id")))
}