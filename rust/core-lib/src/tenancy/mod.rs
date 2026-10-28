//! Tenancy
//!
//! Multi-tenant context management, organization isolation, and tenant resolution.

use crate::config::{Settings, FeatureFlags, RolloutConfig};
use crate::domain::{TenantId, OrganizationId, UserId, Locale, Timestamp};
use crate::errors::{ErrorCode, Result};
use crate::storage::CacheClient;
use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Tenant context with all tenant-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: TenantId,
    pub name: String,
    pub display_name: String,
    pub domain: Option<String>,
    pub subdomain: Option<String>,
    pub status: TenantStatus,
    pub plan: TenantPlan,
    pub settings: TenantSettings,
    pub features: FeatureFlags,
    pub organization_ids: HashSet<OrganizationId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantStatus {
    Active,
    Suspended,
    Pending,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantPlan {
    Free,
    Starter,
    Professional,
    Enterprise,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub default_locale: Locale,
    pub supported_locales: Vec<Locale>,
    pub timezone: String,
    pub theme: ThemeSettings,
    pub branding: BrandingSettings,
    pub security: SecuritySettings,
    pub integrations: IntegrationSettings,
    pub limits: TenantLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub primary_color: String,
    pub secondary_color: String,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub custom_css: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub name: String,
    pub description: Option<String>,
    pub support_email: Option<String>,
    pub support_url: Option<String>,
    pub terms_url: Option<String>,
    pub privacy_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub mfa_required: bool,
    pub password_policy: PasswordPolicySettings,
    pub session_timeout: Duration,
    pub allowed_domains: Vec<String>,
    pub ip_allowlist: Vec<String>,
    pub ip_blocklist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicySettings {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
    pub max_age_days: Option<u32>,
    pub history_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSettings {
    pub lti_enabled: bool,
    pub scorm_enabled: bool,
    pub xapi_enabled: bool,
    pub caliper_enabled: bool,
    pub oneroster_enabled: bool,
    pub sso_providers: Vec<SsoProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoProviderConfig {
    pub provider: String,
    pub enabled: bool,
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantLimits {
    pub max_users: Option<u32>,
    pub max_courses: Option<u32>,
    pub max_storage_gb: Option<u32>,
    pub max_bandwidth_gb: Option<u32>,
    pub max_api_calls_per_hour: Option<u32>,
    pub max_concurrent_sessions: Option<u32>,
}

impl Default for TenantLimits {
    fn default() -> Self {
        Self {
            max_users: None,
            max_courses: None,
            max_storage_gb: None,
            max_bandwidth_gb: None,
            max_api_calls_per_hour: None,
            max_concurrent_sessions: None,
        }
    }
}

/// Tenant resolver trait
#[async_trait::async_trait]
pub trait TenantResolver: Send + Sync {
    async fn resolve(&self, host: &str) -> Result<TenantId>;
    async fn get_context(&self, tenant_id: TenantId) -> Result<TenantContext>;
    async fn get_by_domain(&self, domain: &str) -> Result<Option<TenantContext>>;
    async fn get_by_subdomain(&self, subdomain: &str) -> Result<Option<TenantContext>>;
    async fn list(&self, filter: TenantFilter) -> Result<Vec<TenantContext>>;
    async fn create(&self, input: CreateTenantInput) -> Result<TenantContext>;
    async fn update(&self, tenant_id: TenantId, input: UpdateTenantInput) -> Result<TenantContext>;
    async fn suspend(&self, tenant_id: TenantId) -> Result<()>;
    async fn activate(&self, tenant_id: TenantId) -> Result<()>;
    async fn delete(&self, tenant_id: TenantId) -> Result<()>;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantFilter {
    pub status: Option<TenantStatus>,
    pub plan: Option<TenantPlan>,
    pub search: Option<String>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantInput {
    pub name: String,
    pub display_name: String,
    pub domain: Option<String>,
    pub subdomain: Option<String>,
    pub plan: TenantPlan,
    pub settings: Option<TenantSettings>,
    pub owner_id: UserId,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTenantInput {
    pub display_name: Option<String>,
    pub domain: Option<String>,
    pub subdomain: Option<String>,
    pub plan: Option<TenantPlan>,
    pub status: Option<TenantStatus>,
    pub settings: Option<TenantSettings>,
    pub features: Option<FeatureFlags>,
}

/// Cached tenant resolver implementation
pub struct CachedTenantResolver {
    cache: Arc<dyn CacheClient>,
    db: Arc<dyn crate::http::DatabasePool>,
    default_tenant: TenantId,
    reserved_subdomains: HashSet<String>,
    tenant_cache: DashMap<TenantId, TenantContext>,
    domain_cache: DashMap<String, TenantId>,
    subdomain_cache: DashMap<String, TenantId>,
}

impl CachedTenantResolver {
    pub fn new(
        cache: Arc<dyn CacheClient>,
        db: Arc<dyn crate::http::DatabasePool>,
        default_tenant: TenantId,
    ) -> Self {
        let mut reserved = HashSet::new();
        reserved.extend([
            "www", "api", "admin", "app", "studio", "docs", "blog", "support",
            "status", "cdn", "assets", "static", "media", "files", "uploads",
            "auth", "login", "register", "password", "reset", "verify",
            "health", "ready", "live", "metrics", "debug", "test", "staging",
            "prod", "production", "dev", "development", "local", "localhost",
        ].into_iter().map(String::from));

        Self {
            cache,
            db,
            default_tenant,
            reserved_subdomains: reserved,
            tenant_cache: DashMap::new(),
            domain_cache: DashMap::new(),
            subdomain_cache: DashMap::new(),
        }
    }

    pub fn is_reserved_subdomain(&self, subdomain: &str) -> bool {
        self.reserved_subdomains.contains(subdomain)
    }

    fn cache_key(tenant_id: TenantId) -> String {
        format!("tenant:{}", tenant_id)
    }

    fn domain_cache_key(domain: &str) -> String {
        format!("tenant:domain:{}", domain)
    }

    fn subdomain_cache_key(subdomain: &str) -> String {
        format!("tenant:subdomain:{}", subdomain)
    }
}

#[async_trait::async_trait]
impl TenantResolver for CachedTenantResolver {
    async fn resolve(&self, host: &str) -> Result<TenantId> {
        // Remove port if present
        let host = host.split(':').next().unwrap_or(host);

        // Check cache first
        if let Some(cached) = self.domain_cache.get(host) {
            return Ok(*cached);
        }

        if let Some(cached) = self.subdomain_cache.get(host) {
            return Ok(*cached);
        }

        // Try to resolve from database
        // This would query the database for matching domain/subdomain
        // For now, return default tenant
        Ok(self.default_tenant)
    }

    async fn get_context(&self, tenant_id: TenantId) -> Result<TenantContext> {
        // Check memory cache
        if let Some(ctx) = self.tenant_cache.get(&tenant_id) {
            return Ok(ctx.clone());
        }

        // Check Redis cache
        let cache_key = Self::cache_key(tenant_id);
        if let Some(data) = self.cache.get(&cache_key).await? {
            let context: TenantContext = serde_json::from_slice(&data)?;
            self.tenant_cache.insert(tenant_id, context.clone());
            return Ok(context);
        }

        // Load from database
        // This would query the database
        // For now, return a default context
        let context = TenantContext {
            tenant_id,
            name: "default".to_string(),
            display_name: "Default Tenant".to_string(),
            domain: None,
            subdomain: None,
            status: TenantStatus::Active,
            plan: TenantPlan::Free,
            settings: TenantSettings {
                default_locale: Locale::default(),
                supported_locales: vec![Locale::default()],
                timezone: "UTC".to_string(),
                theme: ThemeSettings {
                    primary_color: "#2563eb".to_string(),
                    secondary_color: "#1e40af".to_string(),
                    logo_url: None,
                    favicon_url: None,
                    custom_css: None,
                },
                branding: BrandingSettings {
                    name: "Axel LMS".to_string(),
                    description: None,
                    support_email: None,
                    support_url: None,
                    terms_url: None,
                    privacy_url: None,
                },
                security: SecuritySettings {
                    mfa_required: false,
                    password_policy: PasswordPolicySettings {
                        min_length: 12,
                        require_uppercase: true,
                        require_lowercase: true,
                        require_numbers: true,
                        require_symbols: true,
                        max_age_days: Some(90),
                        history_count: 5,
                    },
                    session_timeout: Duration::from_secs(86400 * 30),
                    allowed_domains: vec![],
                    ip_allowlist: vec![],
                    ip_blocklist: vec![],
                },
                integrations: IntegrationSettings {
                    lti_enabled: true,
                    scorm_enabled: true,
                    xapi_enabled: true,
                    caliper_enabled: true,
                    oneroster_enabled: false,
                    sso_providers: vec![],
                },
                limits: TenantLimits::default(),
            },
            features: FeatureFlags {
                flags: HashMap::new(),
                rollout: HashMap::new(),
            },
            organization_ids: HashSet::new(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            deleted_at: None,
        };

        // Cache it
        self.tenant_cache.insert(tenant_id, context.clone());
        
        let data = serde_json::to_vec(&context)?;
        self.cache.set(&cache_key, &data, Duration::from_secs(3600)).await?;

        Ok(context)
    }

    async fn get_by_domain(&self, domain: &str) -> Result<Option<TenantContext>> {
        // Check cache
        let cache_key = Self::domain_cache_key(domain);
        if let Some(tenant_id) = self.domain_cache.get(domain) {
            return Ok(Some(self.get_context(*tenant_id).await?));
        }

        // Query database
        // For now, return None
        Ok(None)
    }

    async fn get_by_subdomain(&self, subdomain: &str) -> Result<Option<TenantContext>> {
        // Check cache
        let cache_key = Self::subdomain_cache_key(subdomain);
        if let Some(tenant_id) = self.subdomain_cache.get(subdomain) {
            return Ok(Some(self.get_context(*tenant_id).await?));
        }

        // Query database
        Ok(None)
    }

    async fn list(&self, filter: TenantFilter) -> Result<Vec<TenantContext>> {
        // Query database with filter
        Ok(vec![])
    }

    async fn create(&self, input: CreateTenantInput) -> Result<TenantContext> {
        // Validate subdomain
        if let Some(subdomain) = &input.subdomain {
            if self.is_reserved_subdomain(subdomain) {
                return Err(crate::errors::ApexosError::new(ErrorCode::SubdomainReserved, "Subdomain is reserved"));
            }
        }

        // Create in database
        // For now, return a new context
        let tenant_id = TenantId::new();
        let now = Timestamp::now();
        
        let context = TenantContext {
            tenant_id,
            name: input.name,
            display_name: input.display_name,
            domain: input.domain,
            subdomain: input.subdomain,
            status: TenantStatus::Pending,
            plan: input.plan,
            settings: input.settings.unwrap_or_else(|| TenantSettings {
                default_locale: Locale::default(),
                supported_locales: vec![Locale::default()],
                timezone: "UTC".to_string(),
                theme: ThemeSettings {
                    primary_color: "#2563eb".to_string(),
                    secondary_color: "#1e40af".to_string(),
                    logo_url: None,
                    favicon_url: None,
                    custom_css: None,
                },
                branding: BrandingSettings {
                    name: "Axel LMS".to_string(),
                    description: None,
                    support_email: None,
                    support_url: None,
                    terms_url: None,
                    privacy_url: None,
                },
                security: SecuritySettings {
                    mfa_required: false,
                    password_policy: PasswordPolicySettings {
                        min_length: 12,
                        require_uppercase: true,
                        require_lowercase: true,
                        require_numbers: true,
                        require_symbols: true,
                        max_age_days: Some(90),
                        history_count: 5,
                    },
                    session_timeout: Duration::from_secs(86400 * 30),
                    allowed_domains: vec![],
                    ip_allowlist: vec![],
                    ip_blocklist: vec![],
                },
                integrations: IntegrationSettings {
                    lti_enabled: true,
                    scorm_enabled: true,
                    xapi_enabled: true,
                    caliper_enabled: true,
                    oneroster_enabled: false,
                    sso_providers: vec![],
                },
                limits: TenantLimits::default(),
            }),
            features: FeatureFlags {
                flags: HashMap::new(),
                rollout: HashMap::new(),
            },
            organization_ids: HashSet::new(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        // Cache it
        self.tenant_cache.insert(tenant_id, context.clone());
        let cache_key = Self::cache_key(tenant_id);
        let data = serde_json::to_vec(&context)?;
        self.cache.set(&cache_key, &data, Duration::from_secs(3600)).await?;

        if let Some(domain) = &context.domain {
            self.domain_cache.insert(domain.clone(), tenant_id);
        }
        if let Some(subdomain) = &context.subdomain {
            self.subdomain_cache.insert(subdomain.clone(), tenant_id);
        }

        Ok(context)
    }

    async fn update(&self, tenant_id: TenantId, input: UpdateTenantInput) -> Result<TenantContext> {
        let mut context = self.get_context(tenant_id).await?;

        if let Some(display_name) = input.display_name {
            context.display_name = display_name;
        }
        if let Some(domain) = input.domain {
            context.domain = Some(domain);
        }
        if let Some(subdomain) = input.subdomain {
            if self.is_reserved_subdomain(&subdomain) {
                return Err(crate::errors::ApexosError::new(ErrorCode::SubdomainReserved, "Subdomain is reserved"));
            }
            context.subdomain = Some(subdomain);
        }
        if let Some(plan) = input.plan {
            context.plan = plan;
        }
        if let Some(status) = input.status {
            context.status = status;
        }
        if let Some(settings) = input.settings {
            context.settings = settings;
        }
        if let Some(features) = input.features {
            context.features = features;
        }

        context.updated_at = Timestamp::now();

        // Update cache
        self.tenant_cache.insert(tenant_id, context.clone());
        let cache_key = Self::cache_key(tenant_id);
        let data = serde_json::to_vec(&context)?;
        self.cache.set(&cache_key, &data, Duration::from_secs(3600)).await?;

        // Update domain/subdomain caches
        self.domain_cache.retain(|_, v| *v != tenant_id);
        self.subdomain_cache.retain(|_, v| *v != tenant_id);
        if let Some(domain) = &context.domain {
            self.domain_cache.insert(domain.clone(), tenant_id);
        }
        if let Some(subdomain) = &context.subdomain {
            self.subdomain_cache.insert(subdomain.clone(), tenant_id);
        }

        Ok(context)
    }

    async fn suspend(&self, tenant_id: TenantId) -> Result<()> {
        let mut context = self.get_context(tenant_id).await?;
        context.status = TenantStatus::Suspended;
        context.updated_at = Timestamp::now();
        
        self.tenant_cache.insert(tenant_id, context.clone());
        let cache_key = Self::cache_key(tenant_id);
        let data = serde_json::to_vec(&context)?;
        self.cache.set(&cache_key, &data, Duration::from_secs(3600)).await?;

        Ok(())
    }

    async fn activate(&self, tenant_id: TenantId) -> Result<()> {
        let mut context = self.get_context(tenant_id).await?;
        context.status = TenantStatus::Active;
        context.updated_at = Timestamp::now();
        
        self.tenant_cache.insert(tenant_id, context.clone());
        let cache_key = Self::cache_key(tenant_id);
        let data = serde_json::to_vec(&context)?;
        self.cache.set(&cache_key, &data, Duration::from_secs(3600)).await?;

        Ok(())
    }

    async fn delete(&self, tenant_id: TenantId) -> Result<()> {
        let mut context = self.get_context(tenant_id).await?;
        context.status = TenantStatus::Deleted;
        context.deleted_at = Some(Timestamp::now());
        context.updated_at = Timestamp::now();
        
        // Remove from caches
        self.tenant_cache.remove(&tenant_id);
        self.domain_cache.retain(|_, v| *v != tenant_id);
        self.subdomain_cache.retain(|_, v| *v != tenant_id);

        Ok(())
    }
}

/// Organization context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationContext {
    pub organization_id: OrganizationId,
    pub tenant_id: TenantId,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub domain: Option<String>,
    pub settings: OrganizationSettings,
    pub member_count: u64,
    pub course_count: u64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSettings {
    pub default_role: String,
    pub allow_self_enrollment: bool,
    pub require_approval: bool,
    pub visibility: OrganizationVisibility,
    pub branding: Option<ThemeSettings>,
    pub integrations: IntegrationSettings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationVisibility {
    Private,
    Members,
    Tenant,
    Public,
}

/// Feature flag evaluation
pub struct FeatureEvaluator {
    resolver: Arc<dyn TenantResolver>,
}

impl FeatureEvaluator {
    pub fn new(resolver: Arc<dyn TenantResolver>) -> Self {
        Self { resolver }
    }

    pub async fn is_enabled(&self, tenant_id: TenantId, flag: &str, user_id: Option<UserId>) -> Result<bool> {
        let context = self.resolver.get_context(tenant_id).await?;
        
        // Check tenant-level flag
        if let Some(&enabled) = context.features.flags.get(flag) {
            return Ok(enabled);
        }

        // Check rollout
        if let Some(rollout) = context.features.rollout.get(flag) {
            return Ok(self.evaluate_rollout(rollout, user_id));
        }

        // Check global defaults
        Ok(false)
    }

    fn evaluate_rollout(&self, rollout: &RolloutConfig, user_id: Option<UserId>) -> bool {
        // Check tenant inclusion
        if !rollout.tenants.is_empty() {
            // Would check if tenant is in rollout.tenants
        }

        // Check user inclusion
        if let Some(user) = user_id {
            if rollout.users.iter().any(|u| *u == user) {
                return true;
            }
        }

        // Check group inclusion
        // Would check user's groups

        // Percentage rollout
        if rollout.percentage > 0 {
            // Deterministic hash-based rollout
            let hash = if let Some(user) = user_id {
                user.as_ulid().to_string()
            } else {
                "anonymous".to_string()
            };
            let hash_value = self.hash_string(&hash);
            return (hash_value % 100) < rollout.percentage as u64;
        }

        false
    }

    fn hash_string(&self, s: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_context_defaults() {
        let context = TenantContext {
            tenant_id: TenantId::new(),
            name: "test".to_string(),
            display_name: "Test Tenant".to_string(),
            domain: None,
            subdomain: None,
            status: TenantStatus::Active,
            plan: TenantPlan::Free,
            settings: TenantSettings {
                default_locale: Locale::default(),
                supported_locales: vec![Locale::default()],
                timezone: "UTC".to_string(),
                theme: ThemeSettings {
                    primary_color: "#2563eb".to_string(),
                    secondary_color: "#1e40af".to_string(),
                    logo_url: None,
                    favicon_url: None,
                    custom_css: None,
                },
                branding: BrandingSettings {
                    name: "Axel LMS".to_string(),
                    description: None,
                    support_email: None,
                    support_url: None,
                    terms_url: None,
                    privacy_url: None,
                },
                security: SecuritySettings {
                    mfa_required: false,
                    password_policy: PasswordPolicySettings {
                        min_length: 12,
                        require_uppercase: true,
                        require_lowercase: true,
                        require_numbers: true,
                        require_symbols: true,
                        max_age_days: Some(90),
                        history_count: 5,
                    },
                    session_timeout: Duration::from_secs(86400 * 30),
                    allowed_domains: vec![],
                    ip_allowlist: vec![],
                    ip_blocklist: vec![],
                },
                integrations: IntegrationSettings {
                    lti_enabled: true,
                    scorm_enabled: true,
                    xapi_enabled: true,
                    caliper_enabled: true,
                    oneroster_enabled: false,
                    sso_providers: vec![],
                },
                limits: TenantLimits::default(),
            },
            features: FeatureFlags {
                flags: HashMap::new(),
                rollout: HashMap::new(),
            },
            organization_ids: HashSet::new(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            deleted_at: None,
        };

        assert_eq!(context.status, TenantStatus::Active);
        assert_eq!(context.plan, TenantPlan::Free);
    }

    #[test]
    fn reserved_subdomains() {
        let resolver = CachedTenantResolver::new(
            Arc::new(MockCache),
            Arc::new(MockDb),
            TenantId::new(),
        );

        assert!(resolver.is_reserved_subdomain("www"));
        assert!(resolver.is_reserved_subdomain("api"));
        assert!(resolver.is_reserved_subdomain("admin"));
        assert!(!resolver.is_reserved_subdomain("myorg"));
    }
}

// Mocks for testing
struct MockCache;
#[async_trait::async_trait]
impl CacheClient for MockCache {
    async fn get(&self, _key: &str) -> Result<Option<Vec<u8>>> { Ok(None) }
    async fn set(&self, _key: &str, _value: &[u8], _ttl: Duration) -> Result<()> { Ok(()) }
    async fn delete(&self, _key: &str) -> Result<()> { Ok(()) }
    async fn exists(&self, _key: &str) -> Result<bool> { Ok(false) }
    async fn increment(&self, _key: &str, _delta: i64) -> Result<i64> { Ok(0) }
}

struct MockDb;
#[async_trait::async_trait]
impl crate::http::DatabasePool for MockDb {
    async fn acquire(&self) -> Result<Box<dyn crate::http::DatabaseConnection>> { Ok(Box::new(MockConn)) }
    async fn health_check(&self) -> Result<()> { Ok(()) }
}

struct MockConn;
#[async_trait::async_trait]
impl crate::http::DatabaseConnection for MockConn {
    async fn execute(&self, _query: &str) -> Result<u64> { Ok(0) }
    async fn query(&self, _query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> { Ok(vec![]) }
    async fn query_one(&self, _query: &str) -> Result<Option<HashMap<String, serde_json::Value>>> { Ok(None) }
    async fn begin_transaction(&self) -> Result<Box<dyn crate::http::Transaction>> { Ok(Box::new(MockTx)) }
}

struct MockTx;
#[async_trait::async_trait]
impl crate::http::Transaction for MockTx {
    async fn execute(&self, _query: &str) -> Result<u64> { Ok(0) }
    async fn query(&self, _query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> { Ok(vec![]) }
    async fn commit(self: Box<Self>) -> Result<()> { Ok(()) }
    async fn rollback(self: Box<Self>) -> Result<()> { Ok(()) }
}