//! Sessions Handlers

use crate::domain::Session;
use crate::services::session::SessionService;
use apexos_core::{
    domain::{SessionId, CorrelationId},
    errors::{ErrorCode, Result, ApexosError},
    extractors::{AuthUser, CorrelationId},
    http::ApiResponse,
    events::types::UserLogout,
};
use axum::{
    extract::{State, Path, Extension, Query},
    response::Json,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, instrument};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SessionResponse {
    pub id: String,
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub device_fingerprint: Option<String>,
    pub location: Option<SessionLocation>,
    pub status: String,
    pub mfa_verified: bool,
    pub remember_me: bool,
    pub created_at: String,
    pub last_activity: String,
    pub current: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SessionLocation {
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SessionsListResponse {
    pub sessions: Vec<SessionResponse>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SessionsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// ============================================================================
// Sessions Handlers
// ============================================================================

/// List active sessions for current user
#[utoipa::path(
    get,
    path = "/api/v1/auth/sessions",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of sessions", body = SessionsListResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "sessions",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, session_service))]
pub async fn list_sessions(
    State(state): State<AppState>,
    Extension(session_service): Extension<Arc<crate::services::session::SessionService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Query(query): Query<SessionsQuery>,
) -> Result<Json<ApiResponse<SessionsListResponse>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    let sessions = session_service.get_user_sessions(user_id).await?;
    
    let session_responses: Vec<SessionResponse> = sessions.into_iter().map(|s| SessionResponse {
        id: s.id.to_string(),
        user_id: s.user_id.to_string(),
        ip_address: s.ip_address,
        user_agent: s.user_agent,
        device_fingerprint: s.device_fingerprint,
        location: s.location.map(|l| SessionLocation {
            country: l.country,
            country_code: l.country_code,
            region: l.region,
            city: l.city,
            latitude: l.latitude,
            longitude: l.longitude,
            timezone: l.timezone,
            isp: l.isp,
            organization: l.organization,
        }),
        status: format!("{:?}", s.status).to_lowercase(),
        mfa_verified: s.mfa_verified,
        remember_me: s.remember_me,
        created_at: s.created_at.to_string(),
        last_activity: s.last_activity.to_string(),
        current: s.id.to_string() == auth_user.claims.session_id.as_deref().unwrap_or(""),
    }).collect();
    
    let response = SessionsListResponse {
        sessions: session_responses,
        total: 0, // Would come from pagination
        page: 1,
        per_page: 20,
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// Revoke a specific session
#[utoipa::path(
    delete,
    path = "/api/v1/auth/sessions/{id}",
    params(
        ("id" = String, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session revoked"),
        (status = 404, description = "Session not found")
    ),
    tag = "sessions",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, session_service))]
pub async fn revoke_session(
    State(state): State<AppState>,
    Extension(session_service): Extension<Arc<crate::services::session::SessionService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let session_id = crate::domain::SessionId::from_string(&id);
    
    // Verify session belongs to current user
    // Would check ownership here
    
    session_service.revoke(
        crate::domain::SessionId::from_string(&id),
        "Revoked by user".to_string(),
    ).await?;
    
    Ok(Json(ApiResponse::new((), "request-id")))
}

/// Revoke all sessions except current
#[utoipa::path(
    delete,
    path = "/api/v1/auth/sessions/all",
    responses(
        (status = 200, description = "All sessions revoked"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "sessions",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, session_service))]
pub async fn revoke_all_sessions(
    State(state): State<AppState>,
    Extension(session_service): Extension<Arc<crate::services::session::SessionService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    // Keep current session if specified
    let current_session_id = auth_user.claims.session_id.as_ref()
        .and_then(|s| crate::domain::SessionId::from_string(s).ok());
    
    session_service.revoke_all_for_user(
        crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap()),
        current_session_id,
    ).await?;
    
    Ok(Json(ApiResponse::new((), "request-id")))
}