//! OAuth2/OIDC Handlers

use crate::domain::{OAuthClient, AuthorizationCode, AccessToken, RefreshToken};
use crate::services::oauth::OAuthService;
use apexos_core::{
    domain::{CorrelationId},
    errors::{ErrorCode, Result, ApexosError},
    http::ApiResponse,
};
use axum::{
    extract::{State, Query, Extension, Form},
    response::{Json, Redirect, Html},
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, warn, instrument};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct AuthorizeRequest {
    #[validate(length(min = 1))]
    pub client_id: String,
    #[validate(url)]
    pub redirect_uri: String,
    #[validate(length(min = 1))]
    pub response_type: String, // "code"
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>, // "S256" or "plain"
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuthorizeResponse {
    pub authorization_code: String,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct TokenRequest {
    #[validate(length(min = 1))]
    pub grant_type: String,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub device_code: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct RevokeTokenRequest {
    #[validate(length(min = 1))]
    pub token: String,
    pub token_type_hint: Option<String>, // "access_token" or "refresh_token"
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct IntrospectTokenRequest {
    #[validate(length(min = 1))]
    pub token: String,
    pub token_type_hint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct IntrospectTokenResponse {
    pub active: bool,
    pub scope: Option<String>,
    pub client_id: Option<String>,
    pub username: Option<String>,
    pub exp: Option<i64>,
    pub iat: Option<i64>,
    pub sub: Option<String>,
    pub token_type: Option<String>,
    pub aud: Option<Vec<String>>,
    pub jti: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UserInfoRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserInfoResponse {
    pub sub: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub preferred_username: Option<String>,
    pub locale: Option<String>,
    pub zoneinfo: Option<String>,
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct Jwk {
    pub kty: String,
    pub use_: String,
    pub kid: String,
    pub alg: String,
    pub n: Option<String>,
    pub e: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
    pub crv: Option<String>,
    pub d: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct JwksRequest {}

// ============================================================================
// OAuth2 Handlers
// ============================================================================

/// OAuth2 Authorization Endpoint
#[utoipa::path(
    get,
    path = "/oauth/authorize",
    params(
        ("client_id" = String, Query, description = "Client ID"),
        ("redirect_uri" = String, Query, description = "Redirect URI"),
        ("response_type" = String, Query, description = "Response type (code)"),
        ("scope" = Option<String>, Query, description = "Requested scopes"),
        ("state" = Option<String>, Query, description = "State parameter"),
        ("code_challenge" = Option<String>, Query, description = "PKCE code challenge"),
        ("code_challenge_method" = Option<String>, Query, description = "Code challenge method (S256)")
    ),
    responses(
        (status = 200, description = "Authorization page"),
        (status = 302, description = "Redirect with authorization code"),
        (status = 400, description = "Invalid request")
    ),
    tag = "oauth2"
)]
#[instrument(skip(state, oauth_service))]
pub async fn authorize(
    State(state): State<AppState>,
    Extension(oauth_service): Extension<Arc<crate::services::oauth::OAuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, ApexosError> {
    // Validate parameters
    let client_id = params.get("client_id")
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_id"))?;
    
    let redirect_uri = params.get("redirect_uri")
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing redirect_uri"))?;
    
    let response_type = params.get("response_type")
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing response_type"))?;
    
    if response_type != "code" {
        return Err(ApexosError::new(ErrorCode::InvalidRequest, "Unsupported response_type"));
    }
    
    // Validate client
    let client = crate::services::oauth::OAuthService::get_client_by_id(&state, client_id).await?
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Invalid client"))?;
    
    // Validate redirect URI
    if !client.validate_redirect_uri(&redirect_uri.parse().unwrap()) {
        return Err(ApexosError::new(ErrorCode::InvalidRequest, "Invalid redirect_uri"));
    }
    
    // Check if user is authenticated
    // If not, redirect to login with OAuth2 parameters preserved
    // For now, return a simple authorization page
    
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Authorize Application</title>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <style>
                body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 400px; margin: 50px auto; padding: 20px; }}
                .btn {{ background: #0c87f2; color: white; border: none; padding: 12px 24px; border-radius: 8px; cursor: pointer; font-size: 16px; width: 100%; }}
                .btn:hover {{ background: #006bd6; }}
                .btn-secondary {{ background: #64748b; }}
                .btn-secondary:hover {{ background: #475569; }}
                .app-info {{ background: #f8fafc; padding: 16px; border-radius: 8px; margin-bottom: 20px; }}
                .app-name {{ font-weight: 600; font-size: 18px; margin-bottom: 4px; }}
                .app-desc {{ color: #64748b; font-size: 14px; }}
                .scopes {{ margin-top: 16px; }}
                .scope-tag {{ display: inline-block; background: #e2e8f0; padding: 4px 8px; border-radius: 4px; font-size: 12px; margin: 2px; }}
            </style>
        </head>
        <body>
            <div class="app-info">
                <div class="app-name">{}</div>
                <div class="app-desc">is requesting access to your account</div>
            </div>
            <div class="scopes">
                <strong>Requested permissions:</strong>
                <div>
                    <span class="scope-tag">openid</span>
                    <span class="scope-tag">profile</span>
                    <span class="scope-tag">email</span>
                </div>
            </div>
            <form method="post" action="/oauth/authorize">
                <input type="hidden" name="client_id" value="{}">
                <input type="hidden" name="redirect_uri" value="{}">
                <input type="hidden" name="response_type" value="code">
                <input type="hidden" name="scope" value="openid profile email">
                <input type="hidden" name="state" value="{}">
                <input type="hidden" name="code_challenge" value="{}">
                <input type="hidden" name="code_challenge_method" value="{}">
                <button type="submit" class="btn">Allow Access</button>
            </form>
            <form method="post" action="/oauth/deny" style="margin-top: 12px;">
                <button type="submit" class="btn btn-secondary">Deny</button>
            </form>
        </body>
        </html>
    "#, 
        "Axel LMS",
        client_id,
        redirect_uri,
        params.get("state").unwrap_or(&"".to_string()),
        params.get("code_challenge").unwrap_or(&"".to_string()),
        params.get("code_challenge_method").unwrap_or(&"S256".to_string())
    );
    
    Ok(Html(html))
}

/// OAuth2 Token Endpoint
#[utoipa::path(
    post,
    path = "/oauth/token",
    request_body = TokenRequest,
    responses(
        (status = 200, description = "Token issued", body = TokenResponse),
        (status = 400, description = "Invalid grant"),
        (status = 401, description = "Invalid client")
    ),
    tag = "oauth2"
)]
#[instrument(skip(state, oauth_service))]
pub async fn token(
    State(state): State<AppState>,
    Extension(oauth_service): Extension<Arc<crate::services::oauth::OAuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Form(payload), _): WithRejection<Form<HashMap<String, String>>, _>,
) -> Result<Json<ApiResponse<TokenResponse>>, ApexosError> {
    let grant_type = payload.get("grant_type")
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing grant_type"))?;
    
    let token_response = match grant_type.as_str() {
        "authorization_code" => {
            let code = payload.get("code")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing code"))?;
            
            let client_id = payload.get("client_id")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_id"))?;
            
            let client_secret = payload.get("client_secret")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_secret"))?;
            
            let redirect_uri = payload.get("redirect_uri")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing redirect_uri"))?;
            
            let token = crate::services::oauth::OAuthService::exchange_code(
                &state,
                code,
                client_id,
                client_secret,
                redirect_uri,
            ).await?;
            
            TokenResponse {
                access_token: token.access_token,
                token_type: "Bearer".to_string(),
                expires_in: 900,
                refresh_token: Some(token.refresh_token.unwrap_or_default()),
                scope: Some("openid profile email".to_string()),
                id_token: token.id_token,
            }
        }
        "refresh_token" => {
            let refresh_token = payload.get("refresh_token")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing refresh_token"))?;
            
            let client_id = payload.get("client_id")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_id"))?;
            
            let client_secret = payload.get("client_secret")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_secret"))?;
            
            let token = crate::services::oauth::OAuthService::refresh_token(
                &state,
                refresh_token,
                client_id,
                client_secret,
            ).await?;
            
            TokenResponse {
                access_token: token.access_token,
                token_type: "Bearer".to_string(),
                expires_in: 900,
                refresh_token: Some(token.refresh_token.unwrap_or_default()),
                scope: Some("openid profile email".to_string()),
                id_token: token.id_token,
            }
        }
        "client_credentials" => {
            let client_id = payload.get("client_id")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_id"))?;
            
            let client_secret = payload.get("client_secret")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_secret"))?;
            
            let scope = payload.get("scope").cloned();
            
            let token = crate::services::oauth::OAuthService::client_credentials(
                &state,
                client_id,
                client_secret,
                scope,
            ).await?;
            
            TokenResponse {
                access_token: token.access_token,
                token_type: "Bearer".to_string(),
                expires_in: 900,
                refresh_token: None,
                scope: scope,
                id_token: None,
            }
        }
        "device_code" => {
            let device_code = payload.get("device_code")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing device_code"))?;
            
            let client_id = payload.get("client_id")
                .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing client_id"))?;
            
            let token = crate::services::oauth::OAuthService::device_token(
                &state,
                device_code,
                client_id,
            ).await?;
            
            TokenResponse {
                access_token: token.access_token,
                token_type: "Bearer".to_string(),
                expires_in: 900,
                refresh_token: Some(token.refresh_token.unwrap_or_default()),
                scope: Some("openid profile email".to_string()),
                id_token: token.id_token,
            }
        }
        _ => return Err(ApexosError::new(ErrorCode::InvalidRequest, "Unsupported grant_type")),
    };
    
    Ok(Json(ApiResponse::new(token_response, "request-id")))
}

/// OAuth2 Token Revocation
#[utoipa::path(
    post,
    path = "/oauth/revoke",
    request_body = RevokeTokenRequest,
    responses(
        (status = 200, description = "Token revoked"),
        (status = 400, description = "Invalid request")
    ),
    tag = "oauth2"
)]
#[instrument(skip(state, oauth_service))]
pub async fn revoke_token(
    State(state): State<AppState>,
    Extension(oauth_service): Extension<Arc<crate::services::oauth::OAuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Form(payload), _): WithRejection<Form<HashMap<String, String>>, _>,
) -> Result<Json<ApiResponse<()>>, ApexosError> {
    let token = payload.get("token")
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing token"))?;
    
    // Would revoke token here
    
    Ok(Json(ApiResponse::new((), "request-id")))
}

/// OAuth2 Token Introspection
#[utoipa::path(
    post,
    path = "/oauth/introspect",
    request_body = IntrospectTokenRequest,
    responses(
        (status = 200, description = "Token introspection result", body = IntrospectTokenResponse),
        (status = 400, description = "Invalid request")
    ),
    tag = "oauth2"
)]
#[instrument(skip(state, oauth_service))]
pub async fn introspect(
    State(state): State<AppState>,
    Extension(oauth_service): Extension<Arc<crate::services::oauth::OAuthService>>,
    Extension(correlation_id): Extension<CorrelationId>,
    WithRejection(Form(payload), _): WithRejection<Form<HashMap<String, String>>, _>,
) -> Result<Json<ApiResponse<IntrospectTokenResponse>>, ApexosError> {
    let token = payload.get("token")
        .ok_or_else(|| ApexosError::new(ErrorCode::InvalidRequest, "Missing token"))?;
    
    // Would introspect token here
    let response = IntrospectTokenResponse {
        active: false,
        scope: None,
        client_id: None,
        username: None,
        exp: None,
        iat: None,
        sub: None,
        token_type: None,
        aud: None,
        jti: None,
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// OAuth2 UserInfo Endpoint
#[utoipa::path(
    get,
    path = "/oauth/userinfo",
    responses(
        (status = 200, description = "User info", body = UserInfoResponse),
        (status = 401, description = "Invalid token")
    ),
    tag = "oauth2",
    security(
        ("bearer_auth" = [])
    )
)]
#[instrument(skip(state, oauth_service))]
pub async fn userinfo(
    State(state): State<AppState>,
    Extension(oauth_service): Extension<Arc<crate::services::oauth::OAuthService>>,
    Extension(auth_user): Extension<AuthUser>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<UserInfoResponse>>, ApexosError> {
    // Extract user info from token
    let response = UserInfoResponse {
        sub: auth_user.claims.sub.clone(),
        name: auth_user.claims.custom.get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        email: auth_user.claims.custom.get("email")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        email_verified: auth_user.claims.custom.get("email_verified")
            .and_then(|v| v.as_bool()),
        preferred_username: auth_user.claims.custom.get("preferred_username")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        locale: auth_user.claims.custom.get("locale")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        zoneinfo: auth_user.claims.custom.get("zoneinfo")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        custom: HashMap::new(),
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// JWKS Endpoint
#[utoipa::path(
    get,
    path = "/.well-known/jwks.json",
    responses(
        (status = 200, description = "JWK Set", body = JwksResponse)
    ),
    tag = "oauth2"
)]
#[instrument(skip(state))]
pub async fn jwks(
    State(state): State<AppState>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<JwksResponse>>, ApexosError> {
    // Would return actual JWKS from key store
    let response = JwksResponse {
        keys: vec![
            Jwk {
                kty: "RSA".to_string(),
                use_: "sig".to_string(),
                kid: "auth-key-1".to_string(),
                alg: "RS256".to_string(),
                n: Some("base64url_encoded_modulus".to_string()),
                e: Some("AQAB".to_string()),
                x: None,
                y: None,
                crv: None,
                d: None,
            }
        ],
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// OpenID Configuration
#[utoipa::path(
    get,
    path = "/.well-known/openid-configuration",
    responses(
        (status = 200, description = "OpenID Configuration")
    ),
    tag = "oauth2"
)]
#[instrument(skip(state))]
pub async fn openid_configuration(
    State(state): State<AppState>,
    Extension(correlation_id): Extension<CorrelationId>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApexosError> {
    let config = json!({
        "issuer": "https://auth.axel.local",
        "authorization_endpoint": "https://auth.axel.local/oauth/authorize",
        "token_endpoint": "https://auth.axel.local/oauth/token",
        "revocation_endpoint": "https://auth.axel.local/oauth/revoke",
        "introspection_endpoint": "https://auth.axel.local/oauth/introspect",
        "userinfo_endpoint": "https://auth.axel.local/oauth/userinfo",
        "jwks_uri": "https://auth.axel.local/.well-known/jwks.json",
        "registration_endpoint": "https://auth.axel.local/oauth/register",
        "scopes_supported": ["openid", "profile", "email", "offline_access"],
        "response_types_supported": ["code", "code id_token", "id_token", "code token", "code id_token token"],
        "response_modes_supported": ["query", "fragment", "form_post"],
        "grant_types_supported": ["authorization_code", "client_credentials", "refresh_token", "urn:ietf:params:oauth:grant-type:device_code"],
        "subject_types_supported": ["public", "pairwise"],
        "id_token_signing_alg_values_supported": ["RS256"],
        "token_endpoint_auth_methods_supported": ["client_secret_basic", "client_secret_post", "client_secret_jwt", "private_key_jwt"],
        "claims_supported": ["sub", "name", "email", "email_verified", "preferred_username", "locale", "zoneinfo"],
        "code_challenge_methods_supported": ["plain", "S256"],
        "device_authorization_endpoint": "https://auth.axel.local/oauth/device",
    });
    
    Ok(Json(ApiResponse::new(config, "request-id")))
}