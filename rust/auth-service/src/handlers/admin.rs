//! Admin Handlers

use crate::domain::{User, UserStatus, RoleId};
use crate::services::auth::AuthService;
use apexos_core::{
    domain::{UserId, TenantId, CorrelationId, AggregateId},
    errors::{ErrorCode, Result, ApexosError},
    extractors::{AuthUser, CorrelationId, TenantExtractor},
    http::ApiResponse,
    rbac::AuthorizationEngine,
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
pub struct AdminUserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub locale: String,
    pub timezone: String,
    pub avatar_url: Option<String>,
    pub status: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub mfa_enabled: bool,
    pub last_login_at: Option<String>,
    pub last_login_ip: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::domain::User> for AdminUserResponse {
    fn from(user: crate::domain::User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.to_string(),
            name: user.name,
            email_verified: user.email_verified,
            phone_verified: user.phone_verified,
            locale: user.locale.to_string(),
            timezone: user.timezone,
            avatar_url: user.avatar_url,
            status: format!("{:?}", user.status).to_lowercase(),
            roles: user.roles.iter().map(|r| r.to_string()).collect(),
            permissions: user.permissions,
            mfa_enabled: user.mfa_enabled,
            last_login_at: user.last_login_at.map(|t| t.to_string()),
            last_login_ip: user.last_login_ip,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AdminUsersListResponse {
    pub users: Vec<AdminUserResponse>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct AdminUsersQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct AdminCreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub locale: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct AdminUpdateUserRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub status: Option<String>,
    pub roles: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct SuspendUserRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuditLogResponse {
    pub id: String,
    pub timestamp: String,
    pub event_type: String,
    pub severity: String,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub ip: String,
    pub action: String,
    pub resource: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuditLogsResponse {
    pub logs: Vec<AuditLogResponse>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct AuditLogsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub user_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// ============================================================================
// Admin Handlers
// ============================================================================

/// List users (admin)
#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("role" = Option<String>, Query, description = "Filter by role"),
        ("search" = Option<String>, Query, description = "Search query")
    ),
    responses(
        (status = 200, description = "List of users", body = AdminUsersListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin required")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn list_users(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    Query(query): Query<AdminUsersQuery>,
) -> Result<Json<ApiResponse<AdminUsersListResponse>>, ApexosError> {
    // Check admin permissions
    if !auth_user.claims.has_role("admin") && !auth_user.claims.has_role("org_admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    let tenant_id = tenant.tenant_id;
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = ((page - 1) * per_page) as u64;
    
    let users = auth_service.list_users(tenant_id, offset, per_page as u64).await?;
    
    let user_responses: Vec<AdminUserResponse> = users.into_iter().map(AdminUserResponse::from).collect();
    
    let response = AdminUsersListResponse {
        users: user_responses,
        total: 0, // Would come from count query
        page,
        per_page,
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// Get user details (admin)
#[utoipa::path(
    get,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = AdminUserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn get_user(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<AdminUserResponse>>, ApexosError> {
    if !auth_user.claims.has_role("admin") && !auth_user.claims.has_role("org_admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    let user_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(user_id);
    let tenant_id = crate::domain::TenantId::new(); // Would come from tenant context
    
    let user = auth_service.get_user(tenant_id, user_id).await?
        .ok_or_else(|| ApexosError::new(ErrorCode::NotFound, "User not found"))?;
    
    Ok(Json(ApiResponse::new(AdminUserResponse::from(user), "request-id")))
}

/// Create user (admin)
#[utoipa::path(
    post,
    path = "/api/v1/admin/users",
    request_body = AdminCreateUserRequest,
    responses(
        (status = 201, description = "User created", body = AdminUserResponse),
        (status = 409, description = "Email already exists")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn create_user(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    WithRejection(Json(payload), _): WithRejection<Json<AdminCreateUserRequest>, _>,
) -> Result<Json<ApiResponse<AdminUserResponse>>, ApexosError> {
    if !auth_user.claims.has_role("admin") && !auth_user.claims.has_role("org_admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    let tenant_id = crate::domain::TenantId::new(); // Would come from tenant context
    let locale = payload.locale.and_then(|l| crate::domain::Locale::new(l));
    let timezone = payload.timezone.unwrap_or_else(|| "UTC".to_string());
    
    let user = crate::services::auth::AuthService::create_user(
        &state,
        tenant_id,
        crate::services::auth::CreateUserInput {
            email: payload.email,
            password: payload.password,
            name: payload.name,
            locale,
            timezone,
        },
    ).await?;
    
    Ok(Json(ApiResponse::new(AdminUserResponse::from(user), "request-id")))
}

/// Update user (admin)
#[utoipa::path(
    put,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = AdminUpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = AdminUserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    Path(id): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<AdminUpdateUserRequest>, _>,
) -> Result<Json<ApiResponse<AdminUserResponse>>, ApexosError> {
    if !auth_user.claims.has_role("admin") && !auth_user.claims.has_role("org_admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    let user_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(user_id);
    let tenant_id = crate::domain::TenantId::new();
    
    let locale = payload.locale.and_then(|l| crate::domain::Locale::new(l));
    let timezone = payload.timezone;
    let status = payload.status.and_then(|s| match s.as_str() {
        "active" => Some(crate::domain::UserStatus::Active),
        "inactive" => Some(crate::domain::UserStatus::Inactive),
        "pending_verification" => Some(crate::domain::UserStatus::PendingVerification),
        "suspended" => Some(crate::domain::UserStatus::Suspended),
        "locked" => Some(crate::domain::UserStatus::Locked),
        "deleted" => Some(crate::domain::UserStatus::Deleted),
        _ => None,
    });
    let roles = payload.roles.as_ref().map(|r| r.iter().map(|s| crate::domain::RoleId::from_string(s)).collect());
    let permissions = payload.permissions.clone();
    
    let user = crate::services::auth::AuthService::update_user(
        &state,
        tenant_id,
        crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&id).unwrap()),
        crate::services::auth::UpdateUserInput {
            name: payload.name,
            display_name: payload.display_name,
            avatar_url: payload.avatar_url,
            locale,
            timezone,
            status,
            roles,
            permissions,
        },
    ).await?;
    
    Ok(Json(ApiResponse::new(AdminUserResponse::from(user), "request-id")))
}

/// Delete user (admin)
#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted"),
        (status = 404, description = "User not found")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    if !auth_user.claims.has_role("admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Super admin access required"));
    }
    
    let user_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(user_id);
    let tenant_id = crate::domain::TenantId::new();
    
    crate::services::auth::AuthService::delete_user(
        &state,
        crate::domain::TenantId::new(),
        crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&id).unwrap()),
    ).await?;
    
    Ok(Json(ApiResponse::new((), "request-id")))
}

/// Suspend user (admin)
#[utoipa::path(
    post,
    path = "/api/v1/admin/users/{id}/suspend",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = SuspendUserRequest,
    responses(
        (status = 200, description = "User suspended"),
        (status = 404, description = "User not found")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn suspend_user(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    Path(id): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<SuspendUserRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    if !auth_user.claims.has_role("admin") && !auth_user.claims.has_role("org_admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    let user_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(user_id);
    let tenant_id = crate::domain::TenantId::new();
    
    crate::services::auth::AuthService::suspend_user(
        &state,
        tenant_id,
        crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&id).unwrap()),
    ).await?;
    
    Ok(Json(ApiResponse::new((), "request-id")))
}

/// Activate user (admin)
#[utoipa::path(
    post,
    path = "/api/v1/admin/users/{id}/activate",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User activated"),
        (status = 404, description = "User not found")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn activate_user(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Extension(tenant): Extension<TenantExtractor>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    if !auth_user.claims.has_role("admin") && !auth_user.claims.has_role("org_admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    let user_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApexosError::new(ErrorCode::InvalidRequest, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(user_id);
    let tenant_id = crate::domain::TenantId::new();
    
    crate::services::auth::AuthService::activate_user(
        &state,
        tenant_id,
        crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&id).unwrap()),
    ).await?;
    
    Ok(Json(ApiResponse::new((), "request-id")))
}

/// Get audit logs (admin)
#[utoipa::path(
    get,
    path = "/api/v1/admin/audit-logs",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("event_type" = Option<String>, Query, description = "Filter by event type"),
        ("severity" = Option<String>, Query, description = "Filter by severity"),
        ("user_id" = Option<String>, Query, description = "Filter by user ID"),
        ("start_date" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("end_date" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status = 200, description = "Audit logs", body = AuditLogsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "admin",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state))]
pub async fn audit_logs(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    Query(query): Query<AuditLogsQuery>,
) -> Result<Json<ApiResponse<AuditLogsResponse>>, ApexosError> {
    if !auth_user.claims.has_role("admin") {
        return Err(ApexosError::new(ErrorCode::Forbidden, "Admin access required"));
    }
    
    // Would query audit logs from database
    // For now, return empty response
    
    let response = AuditLogsResponse {
        logs: vec![],
        total: 0,
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(50),
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}