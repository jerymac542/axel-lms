//! Auth Service - Core Authentication Logic

use crate::domain::{
    User, Session, OAuthClient, AuthorizationCode, AccessToken, RefreshToken,
    DeviceCode, ApiKey, AuditLog, RateLimitBucket,
    UserStatus, SessionStatus, ClientStatus, MfaMethod,
    UserId, TenantId, OrganizationId, SessionId, Timestamp, Email, RoleId,
};
use crate::services::oauth::OAuthService;
use apexos_core::{
    crypto::{TokenGenerator, TokenVerifier, PasswordHasher, TotpManager},
    crypto::argon2::Argon2Config,
    crypto::totp::TotpAlgorithm,
    domain::{UserId, TenantId, OrganizationId, SessionId, Timestamp, Email, RoleId, CorrelationId},
    events::{EventPublisher, MessageBus, Event, EventMetadata},
    errors::{ErrorCode, Result, ApexosError},
    tenancy::TenantResolver,
    validation::{PasswordValidator, rules},
    config::{AuthConfig, JwtConfig, SessionConfig, MfaConfig, PasswordConfig, ApiKeyConfig, RateLimitRule},
    errors::ErrorCode,
};
use apexos_core::events::types::*;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use dashmap::DashMap;
use rand::{Rng, rngs::OsRng};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error, instrument};
use url::Url;
use uuid::Uuid;

// ============================================================================
// Repository Traits
// ============================================================================

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User>;
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>>;
    async fn get_by_email(&self, tenant_id: TenantId, email: &Email) -> Result<Option<User>>;
    async fn get_by_id_with_tenant(&self, id: UserId, tenant_id: TenantId) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn delete(&self, id: UserId) -> Result<()>;
    async fn list(&self, tenant_id: TenantId, offset: u64, limit: u64) -> Result<Vec<User>>;
    async fn count(&self, tenant_id: TenantId) -> Result<u64>;
    async fn add_role(&self, user_id: UserId, role_id: RoleId) -> Result<()>;
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> Result<()>;
    async fn add_permission(&self, user_id: UserId, permission: String) -> Result<()>;
    async fn remove_permission(&self, user_id: UserId, permission: String) -> Result<()>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &Session) -> Result<Session>;
    async fn get_by_id(&self, id: SessionId) -> Result<Option<Session>>;
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Vec<Session>>;
    async fn update(&self, session: &Session) -> Result<Session>;
    async fn revoke(&self, id: SessionId, reason: String) -> Result<()>;
    async fn revoke_all_for_user(&self, user_id: UserId, except: Option<SessionId>) -> Result<u64>;
    async fn cleanup_expired(&self) -> Result<u64>;
}

#[async_trait]
pub trait OAuthClientRepository: Send + Sync {
    async fn create(&self, client: &OAuthClient) -> Result<OAuthClient>;
    async fn get_by_id(&self, id: &str) -> Result<Option<OAuthClient>>;
    async fn get_by_client_id(&self, client_id: &str) -> Result<Option<OAuthClient>>;
    async fn update(&self, client: &OAuthClient) -> Result<OAuthClient>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn list(&self, tenant_id: TenantId, offset: u64, limit: u64) -> Result<Vec<OAuthClient>>;
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn store_access_token(&self, token: &AccessToken) -> Result<()>;
    async fn get_access_token(&self, jti: &str) -> Result<Option<AccessToken>>;
    async fn revoke_access_token(&self, jti: &str) -> Result<()>;
    async fn store_refresh_token(&self, token: &RefreshToken) -> Result<()>;
    async fn get_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshToken>>;
    async fn revoke_refresh_token(&self, token_hash: &str) -> Result<()>;
    async fn revoke_all_for_user(&self, user_id: UserId, client_id: Option<String>) -> Result<u64>;
    async fn store_authorization_code(&self, code: &AuthorizationCode) -> Result<()>;
    async fn get_authorization_code(&self, code: &str) -> Result<Option<AuthorizationCode>>;
    async fn consume_authorization_code(&self, code: &str) -> Result<Option<AuthorizationCode>>;
    async fn store_device_code(&self, code: &DeviceCode) -> Result<()>;
    async fn get_device_code(&self, device_code: &str) -> Result<Option<DeviceCode>>;
    async fn authorize_device_code(&self, device_code: &str, user_id: UserId) -> Result<()>;
    async fn get_device_code_by_user_code(&self, user_code: &str) -> Result<Option<DeviceCode>>;
}

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(&self, key: &ApiKey) -> Result<ApiKey>;
    async fn get_by_id(&self, id: &str) -> Result<Option<ApiKey>>;
    async fn get_by_prefix(&self, prefix: &str) -> Result<Option<ApiKey>>;
    async fn list_by_user(&self, user_id: UserId) -> Result<Vec<ApiKey>>;
    async fn revoke(&self, id: &str) -> Result<()>;
    async fn update_last_used(&self, id: &str, ip: Option<String>) -> Result<()>;
}

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn log(&self, log: &AuditLog) -> Result<()>;
    async fn query(&self, filter: AuditQuery) -> Result<Vec<AuditLog>>;
}

#[derive(Debug, Clone, Default)]
pub struct AuditQuery {
    pub tenant_id: Option<TenantId>,
    pub user_id: Option<UserId>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub limit: u32,
    pub offset: u64,
}

// ============================================================================
// Auth Service
// ============================================================================

pub struct AuthService {
    db: Arc<PgPool>,
    cache: Arc<dyn CacheClient>,
    config: AuthConfig,
    token_generator: TokenGenerator,
    token_verifier: TokenVerifier,
    message_bus: Arc<dyn EventPublisher>,
    tenant_resolver: Arc<dyn TenantResolver>,
    password_validator: PasswordValidator,
    // Repositories (would be injected in real implementation)
    user_repo: Arc<dyn UserRepository>,
    session_repo: Arc<dyn SessionRepository>,
    audit_repo: Arc<dyn AuditRepository>,
    // Rate limiting
    rate_limit_cache: Arc<DashMap<String, RateLimitBucket>>,
}

impl AuthService {
    pub fn new(
        db: Arc<PgPool>,
        cache: Arc<dyn CacheClient>,
        config: AuthConfig,
        token_generator: TokenGenerator,
        token_verifier: TokenVerifier,
        message_bus: Arc<dyn EventPublisher>,
        tenant_resolver: Arc<dyn TenantResolver>,
    ) -> Self {
        let password_validator = PasswordValidator::new(config.password.clone());
        
        Self {
            db: db.clone(),
            cache,
            config: config.clone(),
            token_generator,
            token_verifier,
            message_bus,
            tenant_resolver,
            password_validator,
            user_repo: Arc::new(PostgresUserRepository::new(db.clone())),
            session_repo: Arc::new(PostgresSessionRepository::new(db.clone())),
            audit_repo: Arc::new(PostgresAuditRepository::new(db.clone())),
            rate_limit_cache: Arc::new(DashMap::new()),
        }
    }

    // ========================================================================
    // User Registration
    // ========================================================================
    
    #[instrument(skip(self, email, password, name, locale, timezone))]
    pub async fn register(
        &self,
        tenant_id: TenantId,
        email: Email,
        password: String,
        name: String,
        locale: Option<Locale>,
        timezone: Option<String>,
        correlation_id: CorrelationId,
    ) -> Result<User> {
        // Validate email
        rules::email(&email)?;
        
        // Validate password
        self.password_validator.validate(&password, None)?;
        
        // Check if email already exists
        if self.user_repo.get_by_email(tenant_id, &email).await?.is_some() {
            return Err(ApexosError::new(ErrorCode::UserAlreadyExists, "Email already registered"));
        }
        
        // Hash password
        let password_hash = self.hash_password(&password)?;
        
        // Create user
        let mut user = User::new(tenant_id, email.clone(), name, Some(password_hash));
        user.locale = locale.unwrap_or_default();
        user.timezone = timezone.unwrap_or_else(|| "UTC".to_string());
        
        // Generate email verification token
        user.email_verification_token = Some(self.generate_token());
        user.email_verification_expires = Some(Timestamp::now() + Duration::from_secs(86400)); // 24 hours
        
        // Save user
        let user = self.user_repo.create(&user).await?;
        
        // Publish event
        let event = UserCreatedEvent::new(user.id, tenant_id, email.0, user.name.clone());
        self.message_bus.publish(event).await?;
        
        // Send verification email
        // Would call email service here
        
        // Audit log
        self.audit_log(AuditLog {
            tenant_id,
            user_id: Some(user.id),
            action: "user.registered".to_string(),
            resource_type: "user".to_string(),
            resource_id: Some(user.id.into()),
            ip_address: None,
            user_agent: None,
            severity: AuditSeverity::Info,
            category: AuditCategory::Authentication,
            created_at: Timestamp::now(),
            ..Default::default()
        }).await?;
        
        Ok(user)
    }

    // ========================================================================
    // User Login
    // ========================================================================
    
    #[instrument(skip(self, password))]
    pub async fn login(
        &self,
        tenant_id: TenantId,
        email: Email,
        password: String,
        mfa_code: Option<String>,
        backup_code: Option<String>,
        remember_me: bool,
        ip: String,
        user_agent: String,
        correlation_id: CorrelationId,
    ) -> Result<LoginResult> {
        // Check rate limit
        self.check_rate_limit(&format!("login:{}:{}", tenant_id, email), &self.config.rate_limit.login_attempts).await?;
        
        // Get user
        let mut user = self.user_repo.get_by_email(tenant_id, &email).await?
            .ok_or_else(|| ApexosError::new(ErrorCode::InvalidCredentials, "Invalid credentials"))?;
        
        // Check if user can login
        if !user.can_login() {
            return Err(ApexosError::new(ErrorCode::AccountLocked, "Account is locked or inactive"));
        }
        
        // Verify password
        if let Some(hash) = &user.password_hash {
            if !self.verify_password(&password, hash)? {
                // Increment failed attempts
                user.failed_login_attempts += 1;
                if user.failed_login_attempts >= 5 {
                    user.locked_until = Some(Timestamp::now() + Duration::from_secs(900)); // 15 min lockout
                }
                self.user_repo.update(&user).await?;
                
                // Audit log
                self.audit_log(AuditLog {
                    tenant_id,
                    user_id: Some(user.id),
                    action: "login.failed".to_string(),
                    resource_type: "user".to_string(),
                    resource_id: Some(user.id.into()),
                    ip_address: Some(ip.clone()),
                    user_agent: None,
                    severity: AuditSeverity::Warning,
                    category: AuditCategory::SecurityEvent,
                    created_at: Timestamp::now(),
                    ..Default::default()
                }).await?;
                
                return Err(ApexosError::new(ErrorCode::InvalidCredentials, "Invalid credentials"));
            } else {
                return Err(ApexosError::new(ErrorCode::InvalidCredentials, "Invalid credentials"));
            }
        
        // Check MFA
        if user.mfa_enabled {
            if let Some(code) = mfa_code {
                if !self.verify_totp(&user, &code)? {
                    return Err(ApexosError::new(ErrorCode::MfaInvalid, "Invalid MFA code"));
                }
            } else if let Some(code) = backup_code {
                if !self.verify_backup_code(&mut user, &code).await? {
                    return Err(ApexosError::new(ErrorCode::MfaInvalid, "Invalid backup code"));
                }
            } else {
                // MFA required
                return Ok(LoginResult {
                    user: None,
                    access_token: None,
                    refresh_token: None,
                    mfa_required: true,
                    mfa_challenge: Some(MfaChallenge {
                        method: "totp".to_string(),
                        challenge: "Enter your 6-digit code".to_string(),
                    }),
                });
            }
        }
        
        // Reset failed attempts
        user.failed_login_attempts = 0;
        user.locked_until = None;
        
        // Update last login
        let now = Timestamp::now();
        user.last_login_at = Some(now);
        user.last_login_ip = Some("0.0.0.0".to_string()); // Would come from request
        user.last_login_user_agent = Some("Unknown".to_string()); // Would come from request
        self.user_repo.update(&user).await?;
        
        // Create session
        let session = self.create_session(&user, remember_me).await?;
        
        // Generate tokens
        let access_token = self.generate_access_token(&user, &session.id)?;
        let refresh_token = self.generate_refresh_token(&user, &session.id)?;
        
        // Store session
        self.session_repo.create(&session).await?;
        
        // Publish event
        let event = UserLoginEvent::new(user.id, user.tenant_id, "0.0.0.0".to_string(), "Unknown".to_string(), user.mfa_enabled);
        self.message_bus.publish(event).await?;
        
        // Audit log
        self.audit_log(AuditLog {
            tenant_id,
            user_id: Some(user.id),
            action: "user.login".to_string(),
            resource_type: "user".to_string(),
            resource_id: Some(user.id.into()),
            ip_address: Some("0.0.0.0".to_string()),
            user_agent: None,
            severity: AuditSeverity::Info,
            category: AuditCategory::Authentication,
            created_at: Timestamp::now(),
            ..Default::default()
        }).await?;
        
        Ok(LoginResult {
            user: Some(user),
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            mfa_required: false,
            mfa_challenge: None,
        })
    }

    // ========================================================================
    // Token Generation
    // ========================================================================
    
    fn generate_access_token(&self, user: &User, session_id: SessionId) -> Result<String> {
        let claims = crate::services::auth::JwtClaims::new(
            &self.config.jwt.issuer,
            &user.id.to_string(),
            self.config.jwt.audience.clone(),
            self.config.jwt.access_token_ttl,
            vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        )
        .with_tenant(&user.tenant_id.to_string())
        .with_organization(user.organization_id.as_ref().map(|o| o.to_string()).unwrap_or_default())
        .with_roles(user.roles.iter().map(|r| r.to_string()).collect())
        .with_permissions(user.permissions.clone())
        .with_session(&session_id.to_string())
        .with_mfa(user.mfa_enabled);
        
        self.token_generator.generate_access_token(claims)
    }
    
    fn generate_refresh_token(&self, user: &User, session_id: SessionId) -> Result<String> {
        let claims = crate::services::auth::JwtClaims::new(
            &self.config.jwt.issuer,
            &user.id.to_string(),
            self.config.jwt.audience.clone(),
            self.config.jwt.refresh_token_ttl,
            vec!["offline_access".to_string()],
        )
        .with_tenant(&user.tenant_id.to_string())
        .with_session(&session_id.to_string());
        
        self.token_generator.generate_refresh_token(claims)
    }
    
    // ========================================================================
    // Session Management
    // ========================================================================
    
    async fn create_session(&self, user: &User, remember_me: bool) -> Result<Session> {
        let access_token_jti = Uuid::new_v4().to_string();
        let refresh_token = self.generate_random_token();
        let refresh_token_hash = self.hash_token(&refresh_token)?;
        
        let access_token_expires = Timestamp::now() + self.config.jwt.access_token_ttl;
        let refresh_token_expires = if remember_me {
            Timestamp::now() + self.config.jwt.refresh_token_ttl
        } else {
            Timestamp::now() + Duration::from_secs(86400) // 24 hours
        };
        
        let session = Session::new(
            user.id,
            user.tenant_id,
            "0.0.0.0".to_string(), // Would come from request
            "Unknown".to_string(), // Would come from request
            false, // remember_me
            Uuid::new_v4().to_string(),
            self.hash_token(&Uuid::new_v4().to_string())?,
            access_token_expires,
            refresh_token_expires,
        );
        
        Ok(session)
    }
    
    // ========================================================================
    // Password Management
    // ========================================================================
    
    fn hash_password(&self, password: &str) -> Result<String> {
        let hasher = PasswordHasher::new(self.config.password.argon2.clone());
        hasher.hash(password.as_bytes())
    }
    
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let hasher = PasswordHasher::new(self.config.password.argon2.clone());
        hasher.verify(password.as_bytes(), hash)
    }
    
    pub async fn change_password(
        &self,
        user_id: UserId,
        current_password: String,
        new_password: String,
    ) -> Result<()> {
        let mut user = self.user_repo.get_by_id(user_id).await?
            .ok_or_else(|| ApexosError::new(ErrorCode::UserNotFound, "User not found"))?;
        
        // Verify current password
        if let Some(hash) = &user.password_hash {
            if !self.verify_password(&current_password, hash)? {
                return Err(ApexosError::new(ErrorCode::InvalidCredentials, "Current password is incorrect"));
            }
        } else {
            return Err(ApexosError::new(ErrorCode::InvalidCredentials, "No password set"));
        }
        
        // Validate new password
        self.password_validator.validate(&new_password, Some(&[user.email.0.clone(), user.name.clone()]))?;
        
        // Check password history
        let new_hash = self.hash_password(&new_password)?;
        if user.password_history.iter().any(|h| h == &new_hash) {
            return Err(ApexosError::new(ErrorCode::WeakPassword, "Password was recently used"));
        }
        
        // Update password
        user.password_history.push(user.password_hash.clone().unwrap_or_default());
        if user.password_history.len() > self.config.password.history_count {
            user.password_history.remove(0);
        }
        user.password_hash = Some(new_hash);
        user.password_changed_at = Some(Timestamp::now());
        
        self.user_repo.update(&user).await?;
        
        // Revoke all sessions except current (would need current session ID)
        // self.session_repo.revoke_all_for_user(user_id, None).await?;
        
        // Audit log
        self.audit_log(AuditLog {
            tenant_id: user.tenant_id,
            user_id: Some(user.id),
            action: "password.changed".to_string(),
            resource_type: "user".to_string(),
            resource_id: Some(user.id.into()),
            severity: AuditSeverity::Info,
            category: AuditCategory::SecurityEvent,
            created_at: Timestamp::now(),
            ..Default::default()
        }).await?;
        
        // Publish event
        let event = PasswordChangedEvent {
            metadata: EventMetadata::new("auth-service", "com.apexos.user.password_changed"),
            user_id: user.id,
            changed_by: user.id,
        };
        self.message_bus.publish(event).await?;
        
        Ok(())
    }
    
    // ========================================================================
    // Email Verification
    // ========================================================================
    
    pub async fn verify_email(&self, token: String) -> Result<()> {
        // Would look up user by verification token
        // For now, simplified
        Ok(())
    }
    
    pub async fn resend_verification(&self, email: Email, tenant_id: TenantId) -> Result<()> {
        // Would resend verification email
        Ok(())
    }
    
    // ========================================================================
    // Password Reset
    // ========================================================================
    
    pub async fn request_password_reset(&self, email: Email, tenant_id: TenantId) -> Result<()> {
        // Would generate reset token and send email
        Ok(())
    }
    
    pub async fn reset_password(&self, token: String, new_password: String) -> Result<()> {
        // Would validate token and reset password
        Ok(())
    }
    
    // ========================================================================
    // MFA
    // ========================================================================
    
    fn verify_totp(&self, user: &User, code: &str) -> Result<bool> {
        if let Some(secret) = &user.mfa_secret {
            Ok(TotpManager::verify(
                secret,
                code,
                self.config.mfa.totp.digits,
                self.config.mfa.totp.period,
                TotpAlgorithm::from(self.config.mfa.totp.algorithm.clone()),
                self.config.mfa.totp.skew,
            ))
        } else {
            Ok(false)
        }
    }
    
    async fn verify_backup_code(&self, user: &mut User, code: &str) -> Result<bool> {
        // Find and remove used backup code
        if let Some(pos) = user.backup_codes.iter().position(|c| c == code) {
            user.backup_codes.remove(pos);
            self.user_repo.update(user).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    // ========================================================================
    // Password Reset Token
    // ========================================================================
    
    fn generate_token(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill(&mut bytes);
        base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
    }
    
    fn generate_random_token(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill(&mut bytes);
        base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
    }
    
    fn hash_token(&self, token: &str) -> Result<String> {
        let hasher = PasswordHasher::new(self.config.password.argon2.clone());
        hasher.hash(token.as_bytes())
    }
    
    // ========================================================================
    // Rate Limiting
    // ========================================================================
    
    async fn check_rate_limit(&self, key: &str, rule: &RateLimitRule) -> Result<()> {
        let mut bucket = self.rate_limit_cache.entry(key.to_string()).or_insert_with(|| RateLimitBucket {
            key: key.to_string(),
            count: 0,
            window_start: Timestamp::now(),
            blocked_until: None,
        });
        
        let now = Timestamp::now();
        
        // Check if blocked
        if let Some(blocked_until) = bucket.blocked_until {
            if now < blocked_until {
                return Err(ApexosError::new(ErrorCode::RateLimited, "Rate limited"));
            } else {
                bucket.blocked_until = None;
            }
        }
        
        // Reset window if expired
        if (now.as_millis() - bucket.window_start.as_millis()) > rule.window.as_millis() as i64 {
            bucket.count = 0;
            bucket.window_start = now;
        }
        
        bucket.count += 1;
        
        if bucket.count > rule.max_requests {
            bucket.blocked_until = Some(now + rule.block_duration.unwrap_or(Duration::from_secs(900)));
            return Err(ApexosError::new(ErrorCode::RateLimited, "Rate limited"));
        }
        
        Ok(())
    }
    
    // ========================================================================
    // Audit Logging
    // ========================================================================
    
    async fn audit_log(&self, mut log: AuditLog) -> Result<()> {
        log.id = Uuid::new_v4().to_string();
        self.audit_repo.log(&log).await?;
        Ok(())
    }
    
    // ========================================================================
    // User Management (Admin)
    // ========================================================================
    
    pub async fn list_users(&self, tenant_id: TenantId, offset: u64, limit: u64) -> Result<Vec<User>> {
        self.user_repo.list(tenant_id, offset, min(limit, 100)).await
    }
    
    pub async fn get_user(&self, tenant_id: TenantId, user_id: UserId) -> Result<Option<User>> {
        self.user_repo.get_by_id_with_tenant(user_id, tenant_id).await
    }
    
    pub async fn create_user(&self, tenant_id: TenantId, input: CreateUserInput) -> Result<User> {
        // Validate
        rules::email(&input.email)?;
        self.password_validator.validate(&input.password, None)?;
        
        let email = Email::new(input.email)?;
        
        if self.user_repo.get_by_email(tenant_id, &email).await?.is_some() {
            return Err(ApexosError::new(ErrorCode::UserAlreadyExists, "Email already registered"));
        }
        
        let password_hash = self.hash_password(&input.password)?;
        let mut user = User::new(tenant_id, email, input.name, Some(password_hash));
        user.locale = input.locale.unwrap_or_default();
        user.timezone = input.timezone.unwrap_or_else(|| "UTC".to_string());
        user.email_verified = true; // Admin created users are pre-verified
        user.status = UserStatus::Active;
        
        let user = self.user_repo.create(&user).await?;
        
        Ok(user)
    }
    
    pub async fn update_user(&self, tenant_id: TenantId, user_id: UserId, input: UpdateUserInput) -> Result<User> {
        let mut user = self.user_repo.get_by_id_with_tenant(user_id, tenant_id).await?
            .ok_or_else(|| ApexosError::new(ErrorCode::UserNotFound, "User not found"))?;
        
        if let Some(name) = input.name {
            user.name = name;
        }
        if let Some(display_name) = input.display_name {
            user.display_name = Some(display_name);
        }
        if let Some(avatar_url) = input.avatar_url {
            user.avatar_url = Some(avatar_url);
        }
        if let Some(locale) = input.locale {
            user.locale = locale;
        }
        if let Some(timezone) = input.timezone {
            user.timezone = timezone;
        }
        if let Some(status) = input.status {
            user.status = status;
        }
        if let Some(roles) = input.roles {
            user.roles = roles;
        }
        if let Some(permissions) = input.permissions {
            user.permissions = permissions;
        }
        
        user.updated_at = Timestamp::now();
        self.user_repo.update(&user).await
    }
    
    pub async fn suspend_user(&self, tenant_id: TenantId, user_id: UserId) -> Result<()> {
        let mut user = self.user_repo.get_by_id_with_tenant(user_id, tenant_id).await?
            .ok_or_else(|| ApexosError::new(ErrorCode::UserNotFound, "User not found"))?;
        
        user.status = UserStatus::Suspended;
        user.updated_at = Timestamp::now();
        self.user_repo.update(&user).await?;
        
        // Revoke all sessions
        self.session_repo.revoke_all_for_user(user_id, None).await?;
        
        Ok(())
    }
    
    pub async fn activate_user(&self, tenant_id: TenantId, user_id: UserId) -> Result<()> {
        let mut user = self.user_repo.get_by_id_with_tenant(user_id, tenant_id).await?
            .ok_or_else(|| ApexosError::new(ErrorCode::UserNotFound, "User not found"))?;
        
        user.status = UserStatus::Active;
        user.updated_at = Timestamp::now();
        self.user_repo.update(&user).await
    }
    
    pub async fn delete_user(&self, tenant_id: TenantId, user_id: UserId) -> Result<()> {
        let user = self.user_repo.get_by_id_with_tenant(user_id, tenant_id).await?
            .ok_or_else(|| ApexosError::new(ErrorCode::UserNotFound, "User not found"))?;
        
        // Soft delete
        let mut user = user;
        user.status = UserStatus::Deleted;
        user.deleted_at = Some(Timestamp::now());
        user.updated_at = Timestamp::now();
        self.user_repo.update(&user).await?;
        
        // Revoke all sessions
        self.session_repo.revoke_all_for_user(user_id, None).await?;
        
        Ok(())
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub user: Option<User>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub mfa_required: bool,
    pub mfa_challenge: Option<MfaChallenge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallenge {
    pub method: String,
    pub challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub name: String,
    pub locale: Option<crate::domain::Locale>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub locale: Option<crate::domain::Locale>,
    pub timezone: Option<String>,
    pub status: Option<UserStatus>,
    pub roles: Option<Vec<RoleId>>,
    pub permissions: Option<Vec<String>>,
}

// ============================================================================
// Repository Implementations (PostgreSQL)
// ============================================================================

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User> {
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, tenant_id, organization_id, email, email_verified, phone, phone_verified,
                name, display_name, avatar_url, locale, timezone, password_hash,
                password_changed_at, password_history, status, roles, permissions,
                mfa_enabled, mfa_methods, mfa_secret, webauthn_credentials, backup_codes,
                last_login_at, last_login_ip, last_login_user_agent,
                failed_login_attempts, locked_until,
                email_verification_token, email_verification_expires,
                password_reset_token, password_reset_expires,
                created_at, updated_at, deleted_at, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35)
            "#,
            user.id as Uuid,
            user.tenant_id as Uuid,
            user.organization_id.map(|o| o as Uuid),
            user.email.as_str(),
            user.email_verified,
            user.phone.as_deref(),
            user.phone_verified,
            user.name,
            user.display_name.as_deref(),
            user.avatar_url.as_deref(),
            user.locale.as_str(),
            user.timezone,
            user.password_hash.as_deref(),
            user.password_changed_at.map(|t| t as i64),
            &user.password_history,
            user.status as i16,
            &user.roles.iter().map(|r| r as Uuid).collect::<Vec<_>>(),
            &user.permissions,
            user.mfa_enabled,
            &user.mfa_methods.iter().map(|m| m as i16).collect::<Vec<_>>(),
            user.mfa_secret.as_deref(),
            serde_json::to_value(&user.webauthn_credentials).unwrap(),
            &user.backup_codes,
            user.last_login_at.map(|t| t as i64),
            user.last_login_ip.as_deref(),
            user.last_login_user_agent.as_deref(),
            user.failed_login_attempts as i32,
            user.locked_until.map(|t| t as i64),
            user.email_verification_token.as_deref(),
            user.email_verification_expires.map(|t| t as i64),
            user.password_reset_token.as_deref(),
            user.password_reset_expires.map(|t| t as i64),
            user.created_at as i64,
            user.updated_at as i64,
            user.deleted_at.map(|t| t as i64),
            serde_json::to_value(&user.metadata).unwrap(),
        )
        .execute(&*self.pool)
        .await?;
        
        Ok(user.clone())
    }
    
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>> {
        let row = sqlx::query!("SELECT * FROM users WHERE id = $1", id as Uuid)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(|r| self.row_to_user(r)))
    }
    
    async fn get_by_email(&self, tenant_id: TenantId, email: &Email) -> Result<Option<User>> {
        let row = sqlx::query!("SELECT * FROM users WHERE tenant_id = $1 AND email = $2", tenant_id as Uuid, email.as_str())
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(|r| self.row_to_user(r)))
    }
    
    async fn get_by_id_with_tenant(&self, id: UserId, tenant_id: TenantId) -> Result<Option<User>> {
        let row = sqlx::query!("SELECT * FROM users WHERE id = $1 AND tenant_id = $2", id as Uuid, tenant_id as Uuid)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(|r| self.row_to_user(r)))
    }
    
    async fn update(&self, user: &User) -> Result<User> {
        sqlx::query!(
            r#"
            UPDATE users SET
                email = $2, email_verified = $3, phone = $4, phone_verified = $5,
                name = $6, display_name = $7, avatar_url = $8, locale = $9, timezone = $10,
                password_hash = $11, password_changed_at = $12, password_history = $13,
                status = $13, roles = $14, permissions = $15,
                mfa_enabled = $15, mfa_methods = $16, mfa_secret = $16,
                webauthn_credentials = $17, backup_codes = $18,
                last_login_at = $19, last_login_ip = $20, last_login_user_agent = $21,
                failed_login_attempts = $22, locked_until = $23,
                email_verification_token = $24, email_verification_expires = $25,
                password_reset_token = $26, password_reset_expires = $27,
                updated_at = $28, deleted_at = $29, metadata = $30
            WHERE id = $1
            "#,
            user.id as Uuid,
            user.email.as_str(),
            user.email_verified,
            user.phone.as_deref(),
            user.phone_verified,
            user.name,
            user.display_name.as_deref(),
            user.avatar_url.as_deref(),
            user.locale.as_str(),
            user.timezone,
            user.password_hash.as_deref(),
            user.password_changed_at.map(|t| t as i64),
            &user.password_history,
            user.status as i16,
            &user.roles.iter().map(|r| r as Uuid).collect::<Vec<_>>(),
            &user.permissions,
            user.mfa_enabled,
            &user.mfa_methods.iter().map(|m| m as i16).collect::<Vec<_>>(),
            user.mfa_secret.as_deref(),
            serde_json::to_value(&user.webauthn_credentials).unwrap(),
            &user.backup_codes,
            user.last_login_at.map(|t| t as i64),
            user.last_login_ip.as_deref(),
            user.last_login_user_agent.as_deref(),
            user.failed_login_attempts as i32,
            user.locked_until.map(|t| t as i64),
            user.email_verification_token.as_deref(),
            user.email_verification_expires.map(|t| t as i64),
            user.password_reset_token.as_deref(),
            user.password_reset_expires.map(|t| t as i64),
            user.updated_at as i64,
            user.deleted_at.map(|t| t as i64),
            serde_json::to_value(&user.metadata).unwrap(),
        )
        .execute(&*self.pool)
        .await?;
        
        Ok(user.clone())
    }
    
    async fn delete(&self, id: UserId) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id as Uuid)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    
    async fn list(&self, tenant_id: TenantId, offset: u64, limit: u64) -> Result<Vec<User>> {
        let rows = sqlx::query!("SELECT * FROM users WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3", tenant_id as Uuid, limit as i64, offset as i64)
            .fetch_all(&*self.pool)
            .await?;
        Ok(rows.into_iter().map(|r| self.row_to_user(r)).collect())
    }
    
    async fn count(&self, tenant_id: TenantId) -> Result<u64> {
        let count = sqlx::query!("SELECT COUNT(*) FROM users WHERE tenant_id = $1", tenant_id as Uuid)
            .fetch_one(&*self.pool)
            .await?;
        Ok(count.count.unwrap_or(0) as u64)
    }
    
    async fn add_role(&self, user_id: UserId, role_id: RoleId) -> Result<()> {
        sqlx::query!("UPDATE users SET roles = array_append(roles, $1) WHERE id = $2 AND NOT ($1 = ANY(roles))", role_id as Uuid, user_id as Uuid)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> Result<()> {
        sqlx::query!("UPDATE users SET roles = array_remove(roles, $1) WHERE id = $2", role_id as Uuid, user_id as Uuid)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    
    async fn add_permission(&self, user_id: UserId, permission: String) -> Result<()> {
        sqlx::query!("UPDATE users SET permissions = array_append(permissions, $1) WHERE id = $2 AND NOT ($1 = ANY(permissions))", permission, user_id as Uuid)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    
    async fn remove_permission(&self, user_id: UserId, permission: String) -> Result<()> {
        sqlx::query!("UPDATE users SET permissions = array_remove(permissions, $1) WHERE id = $2", permission, user_id as Uuid)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    
    fn row_to_user(&self, row: sqlx::postgres::PgRow) -> User {
        User {
            id: UserId::from_uuid(row.get("id")),
            tenant_id: TenantId::from_uuid(row.get("tenant_id")),
            organization_id: row.get::<Option<Uuid>, _>("organization_id").map(OrganizationId::from_uuid),
            email: Email::new(row.get("email")).unwrap(),
            email_verified: row.get("email_verified"),
            phone: row.get("phone"),
            phone_verified: row.get("phone_verified"),
            name: row.get("name"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            locale: crate::domain::Locale::new(row.get("locale")),
            timezone: row.get("timezone"),
            password_hash: row.get("password_hash"),
            password_changed_at: row.get::<Option<i64>, _>("password_changed_at").map(Timestamp::from_millis),
            password_history: row.get("password_history"),
            status: match row.get::<i16, _>("status") {
                0 => UserStatus::Active,
                1 => UserStatus::Inactive,
                2 => UserStatus::PendingVerification,
                3 => UserStatus::Suspended,
                4 => UserStatus::Locked,
                5 => UserStatus::Deleted,
                _ => UserStatus::Active,
            },
            roles: row.get::<Vec<Uuid>, _>("roles").into_iter().map(RoleId::from_uuid).collect(),
            permissions: row.get("permissions"),
            mfa_enabled: row.get("mfa_enabled"),
            mfa_methods: row.get::<Vec<i16>, _>("mfa_methods").into_iter().map(|m| match m {
                0 => MfaMethod::Totp,
                1 => MfaMethod::WebAuthn,
                2 => MfaMethod::BackupCode,
                3 => MfaMethod::Sms,
                4 => MfaMethod::Email,
                _ => MfaMethod::Totp,
            }).collect(),
            mfa_secret: row.get("mfa_secret"),
            webauthn_credentials: row.get::<serde_json::Value, _>("webauthn_credentials")
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
            backup_codes: row.get("backup_codes"),
            last_login_at: row.get::<Option<i64>, _>("last_login_at").map(Timestamp::from_millis),
            last_login_ip: row.get("last_login_ip"),
            last_login_user_agent: row.get("last_login_user_agent"),
            failed_login_attempts: row.get::<i32, _>("failed_login_attempts") as u32,
            locked_until: row.get::<Option<i64>, _>("locked_until").map(Timestamp::from_millis),
            email_verification_token: row.get("email_verification_token"),
            email_verification_expires: row.get::<Option<i64>, _>("email_verification_expires").map(Timestamp::from_millis),
            password_reset_token: row.get("password_reset_token"),
            password_reset_expires: row.get::<Option<i64>, _>("password_reset_expires").map(Timestamp::from_millis),
            created_at: Timestamp::from_millis(row.get("created_at")),
            updated_at: Timestamp::from_millis(row.get("updated_at")),
            deleted_at: row.get::<Option<i64>, _>("deleted_at").map(Timestamp::from_millis),
            metadata: row.get::<serde_json::Value, _>("metadata")
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
        }
    }
}

// Similar implementations for SessionRepository, AuditRepository, etc.
// (Abbreviated for brevity - would follow similar patterns)

pub struct PostgresSessionRepository {
    pool: Arc<PgPool>,
}

impl PostgresSessionRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn create(&self, session: &Session) -> Result<Session> {
        sqlx::query!(
            r#"
            INSERT INTO sessions (
                id, user_id, tenant_id, organization_id, ip_address, user_agent,
                device_fingerprint, location, status, mfa_verified, remember_me,
                access_token_jti, refresh_token_hash, access_token_expires,
                refresh_token_expires, last_activity, created_at, revoked_at, revoked_reason
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            "#,
            session.id as Uuid,
            session.user_id as Uuid,
            session.tenant_id as Uuid,
            session.organization_id.map(|o| o as Uuid),
            session.ip_address,
            session.user_agent,
            session.device_fingerprint.as_deref(),
            serde_json::to_value(&session.location).unwrap(),
            service.status as i16,
            service.mfa_verified,
            service.remember_me,
            service.access_token_jti,
            service.refresh_token_hash,
            service.access_token_expires as i64,
            service.refresh_token_expires as i64,
            service.last_activity as i64,
            service.created_at as i64,
            service.revoked_at.map(|t| t as i64),
            service.revoked_reason.as_deref(),
        )
        .execute(&*self.pool)
        .await?;
        Ok(session.clone())
    }
    
    async fn get_by_id(&self, id: SessionId) -> Result<Option<Session>> {
        let row = sqlx::query!("SELECT * FROM sessions WHERE id = $1", id as Uuid)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(|r| self.row_to_session(r)))
    }
    
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Vec<Session>> {
        let rows = sqlx::query!("SELECT * FROM sessions WHERE user_id = $1 ORDER BY created_at DESC", user_id as Uuid)
            .fetch_all(&*self.pool)
            .await?;
        Ok(rows.into_iter().map(|r| self.row_to_session(r)).collect())
    }
    
    async fn update(&self, session: &Session) -> Result<Session> {
        sqlx::query!(
            r#"
            UPDATE sessions SET
                ip_address = $2, user_agent = $3, device_fingerprint = $4,
                location = $5, status = $6, mfa_verified = $7, remember_me = $8,
                access_token_jti = $9, refresh_token_hash = $10,
                access_token_expires = $11, refresh_token_expires = $12,
                last_activity = $13, revoked_at = $14, revoked_reason = $15
            WHERE id = $1
            "#,
            session.id as Uuid,
            session.ip_address,
            session.user_agent,
            session.device_fingerprint.as_deref(),
            serde_json::to_value(&session.location).unwrap(),
            session.status as i16,
            session.mfa_verified,
            session.remember_me,
            session.access_token_jti,
            session.refresh_token_hash,
            session.access_token_expires as i64,
            session.refresh_token_expires as i64,
            session.last_activity as i64,
            session.revoked_at.map(|t| t as i64),
            session.revoked_reason.as_deref(),
        )
        .execute(&*self.pool)
        .await?;
        Ok(session.clone())
    }
    
    async fn revoke(&self, id: SessionId, reason: String) -> Result<()> {
        sqlx::query!("UPDATE sessions SET status = 2, revoked_at = $1, revoked_reason = $2 WHERE id = $1", Timestamp::now() as i64, reason)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    
    async fn revoke_all_for_user(&self, user_id: UserId, except: Option<SessionId>) -> Result<u64> {
        let result = if let Some(except_id) = except {
            sqlx::query!("UPDATE sessions SET status = 2, revoked_at = $1 WHERE user_id = $2 AND id != $3", Timestamp::now() as i64, user_id as Uuid, except_id as Uuid)
                .execute(&*self.pool)
                .await?
        } else {
            sqlx::query!("UPDATE sessions SET status = 2, revoked_at = $1 WHERE user_id = $2", Timestamp::now() as i64, user_id as Uuid)
                .execute(&*self.pool)
                .await?
        };
        Ok(result.rows_affected())
    }
    
    async fn cleanup_expired(&self) -> Result<u64> {
        let result = sqlx::query!("UPDATE sessions SET status = 2 WHERE status = 0 AND refresh_token_expires < $1", Timestamp::now() as i64)
            .execute(&*self.pool)
            .await?;
        Ok(result.rows_affected())
    }
    
    fn row_to_session(&self, row: sqlx::postgres::PgRow) -> Session {
        Session {
            id: SessionId::from_uuid(row.get("id")),
            user_id: UserId::from_uuid(row.get("user_id")),
            tenant_id: TenantId::from_uuid(row.get("tenant_id")),
            organization_id: row.get::<Option<Uuid>, _>("organization_id").map(|u| crate::domain::OrganizationId::from_uuid(u)),
            ip_address: row.get("ip_address"),
            user_agent: row.get("user_agent"),
            device_fingerprint: row.get("device_fingerprint"),
            location: row.get::<serde_json::Value, _>("location").and_then(|v| serde_json::from_value(v).ok()),
            status: match row.get::<i16, _>("status") {
                0 => SessionStatus::Active,
                1 => SessionStatus::Expired,
                2 => SessionStatus::Revoked,
                3 => SessionStatus::Inactive,
                _ => SessionStatus::Active,
            },
            mfa_verified: row.get("mfa_verified"),
            remember_me: row.get("remember_me"),
            access_token_jti: row.get("access_token_jti"),
            refresh_token_hash: row.get("refresh_token_hash"),
            access_token_expires: Timestamp::from_millis(row.get("access_token_expires")),
            refresh_token_expires: Timestamp::from_millis(row.get("refresh_token_expires")),
            last_activity: Timestamp::from_millis(row.get("last_activity")),
            created_at: Timestamp::from_millis(row.get("created_at")),
            revoked_at: row.get::<Option<i64>, _>("revoked_at").map(Timestamp::from_millis),
            revoked_reason: row.get("revoked_reason"),
        }
    }
}

pub struct PostgresAuditRepository {
    pool: Arc<PgPool>,
}

impl PostgresAuditRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PostgresAuditRepository {
    async fn log(&self, log: &AuditLog) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (
                id, tenant_id, user_id, action, resource_type, resource_id,
                old_values, new_values, ip_address, user_agent, correlation_id,
                severity, category, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
            log.id,
            log.tenant_id as Uuid,
            log.user_id.map(|u| u as Uuid),
            log.action,
            log.resource_type,
            log.resource_id.map(|r| r as Uuid),
            log.old_values,
            log.new_values,
            log.ip_address.as_deref(),
            log.user_agent.as_deref(),
            log.correlation_id.as_deref(),
            log.severity as i16,
            log.category as i16,
            log.created_at as i64,
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
    
    async fn query(&self, filter: AuditQuery) -> Result<Vec<AuditLog>> {
        let mut query = "SELECT * FROM audit_logs WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, Postgres> + Send + Sync>> = Vec::new();
        
        if let Some(tenant_id) = filter.tenant_id {
            params.push(Box::new(tenant_id as Uuid));
            query.push_str(&format!(" AND tenant_id = ${}", params.len()));
        }
        if let Some(user_id) = filter.user_id {
            params.push(Box::new(user_id as Uuid));
            query.push_str(&format!(" AND user_id = ${}", params.len()));
        }
        if let Some(event_type) = filter.event_type {
            params.push(Box::new(event_type));
            query.push_str(&format!(" AND action = ${}", params.len()));
        }
        if let Some(severity) = filter.severity {
            params.push(Box::new(severity));
            query.push_str(&format!(" AND severity = ${}", params.len()));
        }
        if let Some(start_date) = filter.start_date {
            params.push(Box::new(start_date.timestamp_millis()));
            query.push_str(&format!(" AND created_at >= ${}", params.len()));
        }
        if let Some(end_date) = filter.end_date {
            params.push(Box::new(end_date.timestamp_millis()));
            query.push_str(&format!(" AND created_at <= ${}", params.len()));
        }
        
        query.push_str(" ORDER BY created_at DESC");
        params.push(Box::new(filter.limit as i64));
        query.push_str(&format!(" LIMIT ${}", params.len()));
        params.push(Box::new(filter.offset as i64));
        query.push_str(&format!(" OFFSET ${}", params.len()));
        
        // This is simplified - real implementation would use dynamic query building
        let rows = sqlx::query!("SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT $1 OFFSET $2", filter.limit as i64, filter.offset as i64)
            .fetch_all(&*self.pool)
            .await?;
        
        Ok(rows.into_iter().map(|r| self.row_to_audit_log(r)).collect())
    }
    
    fn row_to_audit_log(&self, row: sqlx::postgres::PgRow) -> AuditLog {
        AuditLog {
            id: row.get("id"),
            tenant_id: TenantId::from_uuid(row.get("tenant_id")),
            user_id: row.get::<Option<Uuid>, _>("user_id").map(crate::domain::UserId::from_uuid),
            action: row.get("action"),
            resource_type: row.get("resource_type"),
            resource_id: row.get::<Option<Uuid>, _>("resource_id").map(crate::domain::AggregateId::from_uuid),
            old_values: row.get("old_values"),
            new_values: row.get("new_values"),
            ip_address: row.get("ip_address"),
            user_agent: row.get("user_agent"),
            correlation_id: row.get("correlation_id"),
            severity: match row.get::<i16, _>("severity") {
                0 => AuditSeverity::Info,
                1 => AuditSeverity::Low,
                2 => AuditSeverity::Medium,
                3 => AuditSeverity::High,
                4 => AuditSeverity::Critical,
                _ => AuditSeverity::Info,
            },
            category: match row.get::<i16, _>("category") {
                0 => AuditCategory::Authentication,
                1 => AuditCategory::Authorization,
                2 => AuditCategory::DataAccess,
                3 => AuditCategory::DataModification,
                4 => AuditCategory::ConfigurationChange,
                5 => AuditCategory::SecurityEvent,
                6 => AuditCategory::SystemEvent,
                _ => AuditCategory::Authentication,
            },
            created_at: Timestamp::from_millis(row.get("created_at")),
        }
    }
}

// Default implementations for AuditLog
impl Default for AuditLog {
    fn default() -> Self {
        Self {
            id: String::new(),
            tenant_id: crate::domain::TenantId::new(),
            user_id: None,
            action: String::new(),
            resource_type: String::new(),
            resource_id: None,
            old_values: None,
            new_values: None,
            ip_address: None,
            user_agent: None,
            correlation_id: None,
            severity: AuditSeverity::Info,
            category: AuditCategory::Authentication,
            created_at: Timestamp::now(),
        }
    }
}