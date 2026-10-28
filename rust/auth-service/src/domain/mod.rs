//! Auth Service Domain Models

use apexos_core::domain::{UserId, TenantId, OrganizationId, RoleId, SessionId, Timestamp, Email, Locale};
use apexos_core::events::{Event, EventMetadata};
use apexos_core::events::types::{UserCreated, UserLogin, UserLogout, PasswordChanged, MfaEnabled};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// User Aggregate
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub tenant_id: TenantId,
    pub organization_id: Option<OrganizationId>,
    pub email: Email,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub name: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub locale: Locale,
    pub timezone: String,
    pub password_hash: Option<String>,
    pub password_changed_at: Option<Timestamp>,
    pub password_history: Vec<String>, // Argon2 hashes
    pub status: UserStatus,
    pub roles: Vec<RoleId>,
    pub permissions: Vec<String>,
    pub mfa_enabled: bool,
    pub mfa_methods: Vec<MfaMethod>,
    pub mfa_secret: Option<String>, // Encrypted TOTP secret
    pub webauthn_credentials: Vec<WebAuthnCredential>,
    pub backup_codes: Vec<String>, // Hashed backup codes
    pub last_login_at: Option<Timestamp>,
    pub last_login_ip: Option<String>,
    pub last_login_user_agent: Option<String>,
    pub failed_login_attempts: u32,
    pub locked_until: Option<Timestamp>,
    pub email_verification_token: Option<String>,
    pub email_verification_expires: Option<Timestamp>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    PendingVerification,
    Suspended,
    Locked,
    Deleted,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::PendingVerification
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MfaMethod {
    Totp,
    WebAuthn,
    BackupCode,
    Sms,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    pub id: String,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: u32,
    pub name: String,
    pub created_at: Timestamp,
    pub last_used_at: Option<Timestamp>,
}

// ============================================================================
// Session Aggregate
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub organization_id: Option<OrganizationId>,
    pub ip_address: String,
    pub user_agent: String,
    pub device_fingerprint: Option<String>,
    pub location: Option<GeoLocation>,
    pub status: SessionStatus,
    pub mfa_verified: bool,
    pub remember_me: bool,
    pub access_token_jti: String,
    pub refresh_token_hash: String,
    pub access_token_expires: Timestamp,
    pub refresh_token_expires: Timestamp,
    pub last_activity: Timestamp,
    pub created_at: Timestamp,
    pub revoked_at: Option<Timestamp>,
    pub revoked_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Expired,
    Revoked,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
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

// ============================================================================
// OAuth2 Client
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient {
    pub id: String,
    pub tenant_id: TenantId,
    pub name: String,
    pub description: Option<String>,
    pub client_id: String,
    pub client_secret_hash: String,
    pub redirect_uris: Vec<Url>,
    pub grant_types: Vec<GrantType>,
    pub response_types: Vec<ResponseType>,
    pub scopes: Vec<String>,
    pub token_endpoint_auth_method: TokenEndpointAuthMethod,
    pub require_pkce: bool,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub id_token_ttl: Duration,
    pub allowed_origins: Vec<String>,
    pub logo_url: Option<String>,
    pub homepage_url: Option<String>,
    pub policy_url: Option<String>,
    pub tos_url: Option<String>,
    pub status: ClientStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    RefreshToken,
    DeviceCode,
    Password,
    Implicit,
    JwtBearer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
    Token,
    IdToken,
    CodeToken,
    CodeIdToken,
    CodeIdTokenToken,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenEndpointAuthMethod {
    ClientSecretBasic,
    ClientSecretPost,
    ClientSecretJwt,
    PrivateKeyJwt,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientStatus {
    Active,
    Inactive,
    Suspended,
    Deleted,
}

// ============================================================================
// OAuth2 Authorization Code
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCode {
    pub code: String,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<CodeChallengeMethod>,
    pub client_id: String,
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub redirect_uri: Url,
    pub scopes: Vec<String>,
    pub nonce: Option<String>,
    pub expires_at: Timestamp,
    pub consumed_at: Option<Timestamp>,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeChallengeMethod {
    Plain,
    S256,
}

// ============================================================================
// OAuth2 Tokens
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub jti: String,
    pub client_id: String,
    pub user_id: Option<UserId>,
    pub tenant_id: TenantId,
    pub scopes: Vec<String>,
    pub expires_at: Timestamp,
    pub created_at: Timestamp,
    pub revoked_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token_hash: String,
    pub jti: String,
    pub client_id: String,
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub scopes: Vec<String>,
    pub expires_at: Timestamp,
    pub created_at: Timestamp,
    pub revoked_at: Option<Timestamp>,
    pub access_token_jti: Option<String>,
    pub rotation_count: u32,
}

// ============================================================================
// Device Code (Device Authorization Grant)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCode {
    pub device_code: String,
    pub user_code: String,
    pub client_id: String,
    pub tenant_id: TenantId,
    pub scopes: Vec<String>,
    pub expires_at: Timestamp,
    pub authorized_at: Option<Timestamp>,
    pub user_id: Option<UserId>,
    pub token_jti: Option<String>,
    pub created_at: Timestamp,
}

// ============================================================================
// API Key
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub name: String,
    pub prefix: String,
    pub key_hash: String,
    pub scopes: Vec<String>,
    pub last_used_at: Option<Timestamp>,
    pub expires_at: Option<Timestamp>,
    pub revoked_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub last_used_ip: Option<String>,
}

// ============================================================================
// Audit Log
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub tenant_id: TenantId,
    pub user_id: Option<UserId>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub correlation_id: Option<String>,
    pub severity: AuditSeverity,
    pub category: AuditCategory,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    ConfigurationChange,
    SecurityEvent,
    SystemEvent,
}

// ============================================================================
// Rate Limiting
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitBucket {
    pub key: String,
    pub count: u32,
    pub window_start: Timestamp,
    pub blocked_until: Option<Timestamp>,
}

// ============================================================================
// Domain Events (re-export from events)
// ============================================================================

pub type UserCreatedEvent = UserCreated;
pub type UserLoginEvent = UserLogin;
pub type UserLogoutEvent = UserLogin;
pub type PasswordChangedEvent = PasswordChanged;
pub type MfaEnabledEvent = MfaEnabled;

impl Event for UserCreatedEvent {}
impl Event for UserLoginEvent {}
impl Event for UserLogoutEvent {}
impl Event for PasswordChangedEvent {}
impl Event for MfaEnabledEvent {}

impl UserCreatedEvent {
    pub fn new(
        user_id: UserId,
        tenant_id: TenantId,
        email: String,
        name: String,
    ) -> Self {
        Self {
            metadata: EventMetadata::new("auth-service", "com.apexos.user.created")
                .with_user(user_id)
                .with_tenant(tenant_id),
            user_id,
            email,
            name,
            locale: Locale::default(),
            timezone: "UTC".to_string(),
        }
    }
}

impl UserLoginEvent {
    pub fn new(
        user_id: UserId,
        tenant_id: TenantId,
        ip: String,
        user_agent: String,
        mfa_used: bool,
    ) -> Self {
        Self {
            metadata: EventMetadata::new("auth-service", "com.apexos.user.login")
                .with_user(user_id)
                .with_tenant(tenant_id),
            user_id,
            ip: ip.parse().unwrap_or_default(),
            user_agent,
            mfa_used,
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

impl User {
    pub fn new(
        tenant_id: TenantId,
        email: Email,
        name: String,
        password_hash: Option<String>,
    ) -> Self {
        let now = Timestamp::now();
        Self {
            id: UserId::new(),
            tenant_id,
            organization_id: None,
            email,
            email_verified: false,
            phone: None,
            phone_verified: false,
            name,
            display_name: None,
            avatar_url: None,
            locale: Locale::default(),
            timezone: "UTC".to_string(),
            password_hash,
            password_changed_at: None,
            password_history: Vec::new(),
            status: UserStatus::PendingVerification,
            roles: Vec::new(),
            permissions: Vec::new(),
            mfa_enabled: false,
            mfa_methods: Vec::new(),
            mfa_secret: None,
            webauthn_credentials: Vec::new(),
            backup_codes: Vec::new(),
            last_login_at: None,
            last_login_ip: None,
            last_login_user_agent: None,
            failed_login_attempts: 0,
            locked_until: None,
            email_verification_token: None,
            email_verification_expires: None,
            password_reset_token: None,
            password_reset_expires: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            metadata: HashMap::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, UserStatus::Active)
    }

    pub fn is_locked(&self) -> bool {
        matches!(self.status, UserStatus::Locked) ||
        self.locked_until.map_or(false, |t| t > Timestamp::now())
    }

    pub fn can_login(&self) -> bool {
        self.is_active() && !self.is_locked()
    }

    pub fn has_role(&self, role: &RoleId) -> bool {
        self.roles.contains(role)
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string()) ||
        self.permissions.contains(&"*".to_string())
    }
}

impl Session {
    pub fn new(
        user_id: UserId,
        tenant_id: TenantId,
        ip_address: String,
        user_agent: String,
        remember_me: bool,
        access_token_jti: String,
        refresh_token_hash: String,
        access_token_expires: Timestamp,
        refresh_token_expires: Timestamp,
    ) -> Self {
        let now = Timestamp::now();
        Self {
            id: SessionId::new(),
            user_id,
            tenant_id,
            organization_id: None,
            ip_address,
            user_agent,
            device_fingerprint: None,
            location: None,
            status: SessionStatus::Active,
            mfa_verified: false,
            remember_me,
            access_token_jti,
            refresh_token_hash,
            access_token_expires,
            refresh_token_expires,
            last_activity: now,
            created_at: now,
            revoked_at: None,
            revoked_reason: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self.status, SessionStatus::Active) &&
        self.refresh_token_expires > Timestamp::now()
    }

    pub fn is_access_token_valid(&self) -> bool {
        self.is_valid() && self.access_token_expires > Timestamp::now()
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Timestamp::now();
    }
}

impl OAuthClient {
    pub fn new(
        tenant_id: TenantId,
        name: String,
        redirect_uris: Vec<Url>,
        grant_types: Vec<GrantType>,
        scopes: Vec<String>,
    ) -> Self {
        let now = Timestamp::now();
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            name,
            description: None,
            client_id: Uuid::new_v4().to_string(),
            client_secret_hash: String::new(),
            redirect_uris,
            grant_types,
            response_types: vec![ResponseType::Code],
            scopes,
            token_endpoint_auth_method: TokenEndpointAuthMethod::ClientSecretBasic,
            require_pkce: true,
            access_token_ttl: Duration::from_secs(900),
            refresh_token_ttl: Duration::from_secs(86400 * 30),
            id_token_ttl: Duration::from_secs(3600),
            allowed_origins: Vec::new(),
            logo_url: None,
            homepage_url: None,
            policy_url: None,
            tos_url: None,
            status: ClientStatus::Active,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            deleted_at: None,
        }
    }

    pub fn validate_redirect_uri(&self, uri: &Url) -> bool {
        self.redirect_uris.iter().any(|u| u == uri)
    }
}

// URL wrapper for serde
use std::fmt;
use url::Url;

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Url::parse(&s).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Duration {
    pub fn from_secs(secs: u64) -> Self {
        Self::from_secs(secs)
    }
    
    pub fn as_secs(&self) -> u64 {
        self.as_secs()
    }
}