//! Auth Service Configuration

use apexos_core::config::{Settings, DatabaseConfig, RedisConfig, NatsConfig, StorageConfig, AuthConfig, JwtConfig, OAuth2Config, SessionConfig, MfaConfig, PasswordConfig, ApiKeyConfig, EmailConfig, SmtpConfig};
use secrecy::SecretString;
use std::time::Duration;
use url::Url;

pub fn auth_service_settings() -> Settings {
    let mut settings = Settings::default();
    
    // Override service-specific settings
    settings.service.name = "auth-service".to_string();
    settings.server.http.address = "0.0.0.0:8001".parse().unwrap();
    settings.server.http.body_limit = 2 * 1024 * 1024; // 2MB for auth
    
    // Auth-specific settings
    settings.auth.jwt = JwtConfig {
        issuer: "apexos-auth".to_string(),
        audience: vec!["axel".to_string()],
        access_token_ttl: Duration::from_secs(900), // 15 min
        refresh_token_ttl: Duration::from_secs(86400 * 30), // 30 days
        id_token_ttl: Duration::from_secs(3600), // 1 hour
        signing_key: SecretString::new("dev-auth-secret-change-in-production-at-least-32-chars".into()),
        verification_key: None,
        algorithm: apexos_core::config::JwtAlgorithm::RS256,
        key_id: Some("auth-key-1".to_string()),
        jwks_uri: None,
        jwks_cache_ttl: Duration::from_secs(3600),
    };
    
    settings.auth.oauth2 = OAuth2Config {
        enabled: true,
        authorization_server_url: "https://auth.axel.local".parse().unwrap(),
        token_endpoint: "https://auth.axel.local/oauth/token".parse().unwrap(),
        revocation_endpoint: Some("https://auth.axel.local/oauth/revoke".parse().unwrap()),
        introspection_endpoint: Some("https://auth.axel.local/oauth/introspect".parse().unwrap()),
        userinfo_endpoint: Some("https://auth.axel.local/oauth/userinfo".parse().unwrap()),
        jwks_uri: Some("https://auth.axel.local/.well-known/jwks.json".parse().unwrap()),
        clients: Default::default(),
        scopes: Default::default(),
        pkce_required: true,
        device_authorization: true,
    };
    
    settings.auth.session = apexos_core::config::SessionConfig {
        ttl: Duration::from_secs(86400 * 30),
        idle_timeout: Duration::from_secs(3600),
        absolute_timeout: Duration::from_secs(86400 * 30),
        rotating: true,
        secure: true,
        http_only: true,
        same_site: apexos_core::config::SameSite::Lax,
        domain: None,
        path: "/".to_string(),
        store: apexos_core::config::SessionStore::Redis,
    };
    
    settings.auth.mfa = MfaConfig {
        enabled: true,
        required_for_roles: vec!["admin".to_string(), "org_admin".to_string(), "instructor".to_string()],
        totp: apexos_core::config::TotpConfig {
            issuer: "Axel LMS".to_string(),
            digits: 6,
            period: 30,
            algorithm: apexos_core::config::TotpAlgorithm::SHA1,
            skew: 1,
            qr_code_size: 256,
        },
        webauthn: apexos_core::config::WebAuthnConfig {
            enabled: true,
            relying_party_id: "axel.local".to_string(),
            relying_party_name: "Axel LMS".to_string(),
            relying_party_origin: "https://axel.local".parse().unwrap(),
            attestation_conveyance: apexos_core::config::AttestationConveyance::None,
            authenticator_attachment: apexos_core::config::AuthenticatorAttachment::CrossPlatform,
            resident_key: apexos_core::config::ResidentKeyRequirement::Preferred,
            user_verification: apexos_core::config::UserVerificationRequirement::Preferred,
        },
        backup_codes: apexos_core::config::BackupCodesConfig {
            enabled: true,
            count: 10,
            length: 8,
        },
        recovery: apexos_core::config::RecoveryConfig {
            enabled: true,
            email_enabled: true,
            sms_enabled: false,
            token_ttl: Duration::from_secs(3600),
        },
    };
    
    settings.auth.password = PasswordConfig {
        min_length: 12,
        max_length: 128,
        require_uppercase: true,
        require_lowercase: true,
        require_numbers: true,
        require_symbols: true,
        forbid_common: true,
        forbid_personal_info: true,
        history_count: 5,
        max_age: Some(Duration::from_secs(86400 * 90)),
        argon2: Argon2Config {
            variant: apexos_core::config::Argon2Variant::Argon2id,
            version: 19,
            memory_cost: 65536,
            time_cost: 3,
            parallelism: 4,
            hash_length: 32,
            salt_length: 16,
        },
    };
    
    settings.auth.api_key = ApiKeyConfig {
        prefix: "axl".to_string(),
        length: 32,
        hash_algorithm: apexos_core::config::HashAlgorithm::Argon2id,
        default_ttl: Some(Duration::from_secs(86400 * 365)),
        max_per_user: 10,
    };
    
    settings.auth.rate_limit = apexos_core::config::AuthRateLimitConfig {
        login_attempts: apexos_core::config::RateLimitRule {
            max_requests: 5,
            window: Duration::from_secs(300),
            block_duration: Some(Duration::from_secs(900)),
        },
        register_attempts: apexos_core::config::RateLimitRule {
            max_requests: 3,
            window: Duration::from_secs(3600),
            block_duration: Some(Duration::from_secs(3600)),
        },
        password_reset_attempts: apexos_core::config::RateLimitRule {
            max_requests: 2,
            window: Duration::from_secs(3600),
            block_duration: Some(Duration::from_secs(3600)),
        },
        mfa_attempts: apexos_core::config::RateLimitRule {
            max_requests: 5,
            window: Duration::from_secs(300),
            block_duration: Some(Duration::from_secs(900)),
        },
        token_refresh_attempts: apexos_core::config::RateLimitRule {
            max_requests: 10,
            window: Duration::from_secs(60),
            block_duration: None,
        },
    };
    
    // Database - auth service uses its own schema
    settings.database.url = SecretString::new("postgresql://postgres:postgres@localhost:5432/axel_auth".into());
    settings.database.max_connections = 20;
    settings.database.migrations_path = std::path::PathBuf::from("migrations");
    
    // Redis
    settings.redis.url = SecretString::new("redis://localhost:6379/1".into());
    settings.redis.database = 1;
    
    // NATS
    settings.nats.urls = vec!["nats://localhost:4222".to_string()];
    
    // Email
    settings.integrations.email = EmailConfig {
        provider: apexos_core::config::EmailProvider::Smtp,
        from_address: "noreply@axel.local".to_string(),
        from_name: "Axel LMS".to_string(),
        reply_to: Some("support@axel.local".to_string()),
        smtp: SmtpConfig {
            host: "localhost".to_string(),
            port: 1025,
            username: "".to_string(),
            password: SecretString::new("".into()),
            tls: false,
            starttls: false,
            auth: false,
        },
        templates_path: std::path::PathBuf::from("templates/email"),
        rate_limit: apexos_core::config::RateLimitRule {
            max_requests: 100,
            window: Duration::from_secs(60),
            block_duration: None,
        },
    };
    
    // Observability
    settings.observability.tracing.service_name = "auth-service".to_string();
    settings.observability.tracing.environment = "development".to_string();
    
    settings
}

pub fn load_settings() -> apexos_core::errors::Result<apexos_core::config::Settings> {
    let mut settings = Settings::load()?;
    
    // Merge with auth service defaults
    let auth_settings = auth_service_settings();
    
    // Override with auth-specific settings
    settings.service = auth_settings.service;
    settings.server = auth_settings.server;
    settings.database = auth_settings.database;
    settings.redis = auth_settings.redis;
    settings.nats = auth_settings.nats;
    settings.auth = auth_settings.auth;
    settings.integrations.email = auth_settings.integrations.email;
    settings.observability = auth_settings.observability;
    
    settings.validate()?;
    Ok(settings)
}