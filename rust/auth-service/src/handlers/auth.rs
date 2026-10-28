//! Auth Handlers - REST API Handlers for Authentication

use crate::domain::{User, LoginResult, CreateUserInput, UpdateUserInput, UserStatus, RoleId};
use crate::services::auth::AuthService;
use apexos_core::{
    crypto::TokenVerifier,
    domain::{UserId, TenantId, OrganizationId, CorrelationId, Email, Locale},
    errors::{ErrorCode, Result, ApexosError},
    events::{EventPublisher, MessageBus},
    extractors::{AuthUser, OptionalAuthUser, TenantExtractor, CorrelationExtractor},
    http::{AppState, ApiResponse},
};
use axum::{
    extract::{State, Path, Query},
    response::Json,
    Extension,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, warn, instrument};
use validator::Validate;
use utoipa::ToSchema;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub mfa_code: Option<String>,
    pub backup_code: Option<String>,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub user: Option<UserResponse>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: i64,
    pub mfa_required: bool,
    pub mfa_challenge: Option<MfaChallengeResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MfaChallengeResponse {
    pub method: String,
    pub challenge: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub locale: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user: UserResponse,
    pub email_verification_required: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
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
    pub mfa_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::domain::User> for UserResponse {
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
            mfa_enabled: user.mfa_enabled,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VerifyEmailRequest {
    #[validate(length(min = 1))]
    pub token: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResendVerificationRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1))]
    pub current_password: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: Vec<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub locale: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub status: Option<String>,
    pub roles: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UsersListResponse {
    pub users: Vec<UserResponse>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UsersQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SuspendUserRequest {
    pub reason: Option<String>,
}

// ============================================================================
// Auth Handlers
// ============================================================================

/// User login
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Rate limited")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn login(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<LoginRequest>, _>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = Email::new(payload.email)?;
    let remember_me = payload.remember_me.unwrap_or(false);
    
    let result = auth_service.login(
        tenant_id,
        email,
        payload.password,
        payload.mfa_code,
        payload.backup_code,
        remember_me.unwrap_or(false),
        "0.0.0.0".to_string(), // Would extract from request
        "Unknown".to_string(), // Would extract from request
        correlation_id,
    ).await?;
    
    let response = LoginResponse {
        user: result.user.map(UserResponse::from),
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 900, // 15 minutes
        mfa_required: result.mfa_required,
        mfa_challenge: result.mfa_challenge.map(|c| MfaChallengeResponse {
            method: c.method,
            challenge: c.challenge,
        }),
    };
    
    Ok(Json(ApiResponse::new(response, &correlation_id.0)))
}

/// User registration
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = RegisterResponse),
        (status = 409, description = "Email already registered"),
        (status = 422, description = "Validation error")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn register(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<RegisterRequest>, _>,
) -> Result<Json<ApiResponse<RegisterResponse>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = Email::new(payload.email)?;
    let locale = payload.locale.and_then(|l| Locale::new(l));
    let timezone = payload.timezone.unwrap_or_else(|| "UTC".to_string());
    
    let user = auth_service.register(
        tenant_id,
        email,
        payload.password,
        payload.name,
        locale,
        Some(timezone),
        correlation_id,
    ).await?;
    
    let response = RegisterResponse {
        user: UserResponse::from(user),
        email_verification_required: true,
    };
    
    Ok(Json(ApiResponse::new(response, &correlation_id.0)))
}

/// Get current user profile
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state))]
pub async fn me(
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<UserResponse>>, ApexosError> {
    let user = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    // Would fetch from database
    // For now, return from claims
    let user_response = UserResponse {
        id: auth_user.claims.sub.clone(),
        email: auth_user.claims.custom.get("email")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        name: auth_user.claims.custom.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string(),
        email_verified: auth_user.claims.custom.get("email_verified")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        phone_verified: auth_user.claims.custom.get("phone_verified")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        locale: auth_user.claims.custom.get("locale")
            .and_then(|v| v.as_str())
            .unwrap_or("en-US")
            .to_string(),
        timezone: auth_user.claims.custom.get("timezone")
            .and_then(|v| v.as_str())
            .unwrap_or("UTC")
            .to_string(),
        avatar_url: None,
        status: "active".to_string(),
        roles: auth_user.claims.roles.clone(),
        mfa_enabled: auth_user.claims.mfa_verified,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(Json(ApiResponse::new(user_response, "request-id")))
}

/// Change password
#[utoipa::path(
    post,
    path = "/api/v1/auth/change-password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 401, description = "Current password incorrect"),
        (status = 422, description = "Validation error")
    ),
    tag = "auth",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn change_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ChangePasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    auth_service.change_password(
        user_id,
        payload.current_password,
        payload.password,
    ).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn refresh_token(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(token_verifier): Extension<Arc<TokenVerifier>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<RefreshTokenRequest>, _>,
) -> Result<Json<ApiResponse<TokenResponse>>, ApexosError> {
    // Verify refresh token
    let token_data = token_verifier.verify(&payload.refresh_token)?;
    
    // Would validate refresh token in database and rotate
    // For now, return new tokens
    let response = TokenResponse {
        access_token: "new_access_token".to_string(),
        refresh_token: "new_refresh_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 900,
        scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
    };
    
    Ok(Json(ApiResponse::new(response, &correlation_id.0)))
}

/// User logout
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, session_service))]
pub async fn logout(
    State(state): State<AppState>,
    Extension(session_service): Extension<Arc<crate::services::session::SessionService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    if let Some(session_id) = auth_user.claims.session_id {
        let session_id = crate::domain::SessionId::from_string(&session_id);
        session_service.revoke(session_id, "User logout".to_string()).await?;
    }
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Verify email
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-email",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified"),
        (status = 400, description = "Invalid or expired token")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn verify_email(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<VerifyEmailRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    auth_service.verify_email(payload.token).await?;
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Resend verification email
#[utoipa::path(
    post,
    path = "/api/v1/auth/resend-verification",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email sent"),
        (status = 404, description = "User not found")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn resend_verification(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ResendVerificationRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = crate::domain::Email::new(payload.email)?;
    
    auth_service.resend_verification(email, tenant_id).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Forgot password
#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn forgot_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ForgotPasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = crate::domain::Email::new(payload.email)?;
    
    auth_service.request_password_reset(email, tenant_id).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Reset password
#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 400, description = "Invalid or expired token"),
        (status = 422, description = "Validation error")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn reset_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ResetPasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    auth_service.reset_password(payload.token, payload.password).await?;
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Resend verification email
#[utoipa::path(
    post,
    path = "/api/v1/auth/resend-verification",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email sent"),
        (status = 404, description = "User not found")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn resend_verification(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ResendVerificationRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = crate::domain::Email::new(payload.email)?;
    
    auth_service.resend_verification(email, tenant_id).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Forgot password
#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn forgot_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ForgotPasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = crate::domain::Email::new(payload.email)?;
    
    auth_service.request_password_reset(email, tenant_id).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Reset password
#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 400, description = "Invalid or expired token"),
        (status = 422, description = "Validation error")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn reset_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ResetPasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    auth_service.reset_password(payload.token, payload.password).await?;
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Change password
#[utoipa::path(
    post,
    path = "/api/v1/auth/change-password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 401, description = "Current password incorrect"),
        (status = 422, description = "Validation error")
    ),
    tag = "auth",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, auth_service))]
pub async fn change_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ChangePasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    auth_service.change_password(
        user_id,
        payload.current_password,
        payload.password,
    ).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn refresh_token(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(token_verifier): Extension<Arc<TokenVerifier>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<RefreshTokenRequest>, _>,
) -> Result<Json<ApiResponse<TokenResponse>>, ApexosError> {
    // Verify refresh token
    let token_data = token_verifier.verify(&payload.refresh_token)?;
    
    // Would validate refresh token in database and rotate
    // For now, return new tokens
    let response = TokenResponse {
        access_token: "new_access_token".to_string(),
        refresh_token: "new_refresh_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 900,
        scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
    };
    
    Ok(Json(ApiResponse::new(response, &correlation_id.0)))
}

/// User logout
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, session_service))]
pub async fn logout(
    State(state): State<AppState>,
    Extension(session_service): Extension<Arc<crate::services::session::SessionService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    if let Some(session_id) = auth_user.claims.session_id {
        let session_id = crate::domain::SessionId::from_string(&session_id);
        session_service.revoke(session_id, "User logout".to_string()).await?;
    }
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Verify email
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-email",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified"),
        (status = 400, description = "Invalid or expired token")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn verify_email(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<VerifyEmailRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    auth_service.verify_email(payload.token).await?;
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Resend verification email
#[utoipa::path(
    post,
    path = "/api/v1/auth/resend-verification",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email sent"),
        (status = 404, description = "User not found")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn resend_verification(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ResendVerificationRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = crate::domain::Email::new(payload.email)?;
    
    auth_service.resend_verification(email, tenant_id).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Forgot password
#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn forgot_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ForgotPasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let tenant_id = state.tenant_resolver.resolve("default").await?;
    let email = crate::domain::Email::new(payload.email)?;
    
    auth_service.request_password_reset(email, tenant_id).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Reset password
#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 400, description = "Invalid or expired token"),
        (status = 422, description = "Validation error")
    ),
    tag = "auth"
)]
#[instrument(skip(state, auth_service))]
pub async fn reset_password(
    State(state): State<AppState>,
    Extension(auth_service): Extension<Arc<crate::services::auth::AuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<ResetPasswordRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    auth_service.reset_password(payload.token, payload.password).await?;
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}