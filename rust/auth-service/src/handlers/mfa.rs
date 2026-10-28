//! MFA Handlers

use crate::domain::{User, MfaMethod};
use crate::services::mfa::MfaService;
use apexos_core::{
    domain::{UserId, CorrelationId},
    errors::{ErrorCode, Result, ApexosError},
    extractors::AuthUser,
    http::ApiResponse,
};
use axum::{
    extract::{State, Extension},
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
pub struct EnableMfaRequest {
    #[validate(length(equal = 6))]
    pub code: String,
    pub method: Option<String>, // "totp" or "webauthn"
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct EnableMfaResponse {
    pub secret: Option<String>,
    pub qr_code_url: Option<String>,
    pub backup_codes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct DisableMfaRequest {
    #[validate(length(min = 1))]
    pub code: String, // Current MFA code for verification
    pub method: String,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct VerifyMfaRequest {
    #[validate(length(equal = 6))]
    pub code: String,
    pub method: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct VerifyMfaResponse {
    pub verified: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct BackupCodesResponse {
    pub codes: Vec<String>,
}

// ============================================================================
// MFA Handlers
// ============================================================================

/// Enable MFA for current user
#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/enable",
    request_body = EnableMfaRequest,
    responses(
        (status = 200, description = "MFA setup initiated", body = EnableMfaResponse),
        (status = 400, description = "Invalid code or MFA already enabled")
    ),
    tag = "mfa",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, mfa_service))]
pub async fn enable_mfa(
    State(state): State<AppState>,
    Extension(mfa_service): Extension<Arc<crate::services::mfa::MfaService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<EnableMfaRequest>, _>,
) -> Result<Json<ApiResponse<EnableMfaResponse>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    let method = payload.method.as_deref().unwrap_or("totp");
    
    let result = match method {
        "totp" => {
            let (secret, qr_code_url) = mfa_service.setup_totp(&auth_user.claims.sub).await?;
            mfa_service.verify_and_enable_totp(user_id, &payload.code).await?;
            
            let backup_codes = mfa_service.generate_backup_codes(user_id).await?;
            
            EnableMfaResponse {
                secret: Some(secret),
                qr_code_url: Some(qr_code_url),
                backup_codes: Some(backup_codes),
            }
        }
        "webauthn" => {
            // WebAuthn setup would go here
            EnableMfaResponse {
                secret: None,
                qr_code_url: None,
                backup_codes: None,
            }
        }
        _ => return Err(ApexosError::new(ErrorCode::InvalidRequest, "Invalid MFA method")),
    };
    
    Ok(Json(ApiResponse::new(result, &correlation_id.0)))
}

/// Disable MFA for current user
#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/disable",
    request_body = DisableMfaRequest,
    responses(
        (status = 200, description = "MFA disabled"),
        (status = 400, description = "Invalid code")
    ),
    tag = "mfa",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, mfa_service))]
pub async fn disable_mfa(
    State(state): State<AppState>,
    Extension(mfa_service): Extension<Arc<crate::services::mfa::MfaService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<DisableMfaRequest>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    mfa_service.disable_mfa(user_id, &payload.method, &payload.code).await?;
    
    Ok(Json(ApiResponse::new((), &correlation_id.0)))
}

/// Verify MFA code
#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/verify",
    request_body = VerifyMfaRequest,
    responses(
        (status = 200, description = "MFA verification result", body = VerifyMfaResponse),
        (status = 400, description = "Invalid code")
    ),
    tag = "mfa",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, mfa_service))]
pub async fn verify_mfa(
    State(state): State<AppState>,
    Extension(mfa_service): Extension<Arc<crate::services::mfa::MfaService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Json(payload), _): WithRejection<Json<VerifyMfaRequest>, _>,
) -> Result<Json<ApiResponse<VerifyMfaResponse>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    let verified = mfa_service.verify_code(user_id, &payload.method, &payload.code).await?;
    
    let response = VerifyMfaResponse { verified };
    
    Ok(Json(ApiResponse::new(response, &correlation_id.0)))
}

/// Generate backup codes
#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/backup-codes",
    responses(
        (status = 200, description = "Backup codes generated", body = BackupCodesResponse),
        (status = 400, description = "MFA not enabled")
    ),
    tag = "mfa",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, mfa_service))]
pub async fn generate_backup_codes(
    State(state): State<AppState>,
    Extension(mfa_service): Extension<Arc<crate::services::mfa::MfaService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<BackupCodesResponse>>, ApexosError> {
    let user_id = auth_user.claims.sub.parse::<uuid::Uuid>()
        .map_err(|_| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid user ID"))?;
    
    let user_id = crate::domain::UserId::from_uuid(uuid::Uuid::parse_str(&user_id).unwrap());
    
    let codes = mfa_service.generate_backup_codes(user_id).await?;
    
    let response = BackupCodesResponse { codes };
    
    Ok(Json(ApiResponse::new(response, &correlation_id.0)))
}