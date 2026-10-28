//! Configuration Management
//!
//! Centralized configuration with environment variable support, secrets management,
//! and validation for all Apexos services.

use crate::domain::{Locale, TenantId};
use crate::errors::{ErrorCode, Result};
use config::{Config, Environment, File, FileFormat};
use figment::{providers::Format, Figment};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

/// Main settings structure for all services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub service: ServiceConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
    pub kafka: KafkaConfig,
    pub storage: StorageConfig,
    pub auth: AuthConfig,
    pub observability: ObservabilityConfig,
    pub security: SecurityConfig,
    pub features: FeatureFlags,
    pub limits: LimitsConfig,
    pub integrations: IntegrationsConfig,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub instance_id: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
    Test,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub http: HttpServerConfig,
    pub grpc: Option<GrpcServerConfig>,
    pub websocket: Option<WebSocketConfig>,
    pub shutdown_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpServerConfig {
    pub address: SocketAddr,
    pub workers: Option<usize>,
    pub max_connections: Option<usize>,
    pub request_timeout: Duration,
    pub keep_alive_timeout: Duration,
    pub body_limit: usize,
    pub tls: Option<TlsConfig>,
    pub cors: CorsConfig,
    pub compression: bool,
    pub trusted_proxies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcServerConfig {
    pub address: SocketAddr,
    pub max_concurrent_streams: usize,
    pub max_message_size: usize,
    pub tls: Option<TlsConfig>,
    pub reflection: bool,
    pub health_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    pub max_message_size: usize,
    pub max_frame_size: usize,
    pub ping_interval: Duration,
    pub ping_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub ca_path: Option<PathBuf>,
    pub require_client_cert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub exposed_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: Duration,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"]
                .into_iter()
                .map(String::from)
                .collect(),
            allowed_headers: vec!["*".to_string()],
            exposed_headers: vec![],
            allow_credentials: true,
            max_age: Duration::from_secs(86400),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: SecretString,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub acquire_timeout: Duration,
    pub sqlx_offline: bool,
    pub migrations_path: PathBuf,
    pub read_replicas: Vec<SecretString>,
    pub statement_cache_capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: SecretString,
    pub cluster_urls: Vec<SecretString>,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub command_timeout: Duration,
    pub pool_size: usize,
    pub enable_cluster: bool,
    pub enable_sentinel: bool,
    pub sentinel_master: Option<String>,
    pub sentinel_urls: Vec<SecretString>,
    pub tls: bool,
    pub username: Option<String>,
    pub password: Option<SecretString>,
    pub database: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsConfig {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub password: Option<SecretString>,
    pub token: Option<SecretString>,
    pub nkey: Option<SecretString>,
    pub tls: bool,
    pub tls_config: Option<TlsConfig>,
    pub max_reconnects: usize,
    pub reconnect_delay: Duration,
    pub ping_interval: Duration,
    pub max_pending: usize,
    pub jetstream: JetStreamConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JetStreamConfig {
    pub enabled: bool,
    pub domain: Option<String>,
    pub api_prefix: String,
    pub stream_config: StreamConfig,
    pub consumer_config: ConsumerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub name: String,
    pub subjects: Vec<String>,
    pub retention: RetentionPolicy,
    pub max_consumers: i64,
    pub max_msgs: i64,
    pub max_bytes: i64,
    pub max_age: Duration,
    pub max_msg_size: i64,
    pub storage: StorageType,
    pub replicas: i64,
    pub no_ack: bool,
    pub duplicate_window: Duration,
    pub placement: Option<PlacementConfig>,
    pub mirror: Option<MirrorConfig>,
    pub sources: Vec<SourceConfig>,
    pub allow_rollup: bool,
    pub deny_delete: bool,
    deny_purge: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RetentionPolicy {
    Limits,
    Interest,
    WorkQueue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementConfig {
    pub cluster: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorConfig {
    pub name: String,
    pub external: ExternalStreamConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalStreamConfig {
    pub api_prefix: String,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub external: ExternalStreamConfig,
    pub start_seq: Option<u64>,
    pub start_time: Option<String>,
    pub filter_subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerConfig {
    pub durable_name: Option<String>,
    pub ack_policy: AckPolicy,
    pub ack_wait: Duration,
    pub max_deliver: i64,
    pub filter_subject: Option<String>,
    pub replay_policy: ReplayPolicy,
    pub rate_limit: u64,
    pub max_ack_pending: i64,
    pub flow_control: bool,
    pub heartbeat: Duration,
    pub headers_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AckPolicy {
    None,
    All,
    Explicit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReplayPolicy {
    Instant,
    Original,
    New,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub client_id: String,
    pub group_id: String,
    pub security_protocol: SecurityProtocol,
    pub sasl_mechanism: Option<SaslMechanism>,
    pub sasl_username: Option<String>,
    pub sasl_password: Option<SecretString>,
    pub ssl_ca_path: Option<PathBuf>,
    pub ssl_cert_path: Option<PathBuf>,
    pub ssl_key_path: Option<PathBuf>,
    pub enable_auto_commit: bool,
    pub auto_offset_reset: AutoOffsetReset,
    pub session_timeout: Duration,
    pub heartbeat_interval: Duration,
    pub max_poll_records: i32,
    pub max_poll_interval: Duration,
    pub fetch_min_bytes: i32,
    pub fetch_max_wait: Duration,
    pub producer: ProducerConfig,
    pub consumer: ConsumerConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SecurityProtocol {
    Plaintext,
    Ssl,
    SaslPlaintext,
    SaslSsl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SaslMechanism {
    Plain,
    ScramSha256,
    ScramSha512,
    OAuthBearer,
    Gssapi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoOffsetReset {
    Earliest,
    Latest,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducerConfig {
    pub acks: Acks,
    pub retries: i32,
    pub enable_idempotence: bool,
    pub max_in_flight: i32,
    pub compression: CompressionType,
    pub linger_ms: i32,
    pub batch_size: i32,
    pub buffer_memory: i64,
    pub delivery_timeout: Duration,
    pub request_timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Acks {
    None,
    One,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConsumerConfig {
    pub fetch_min_bytes: i32,
    pub fetch_max_wait: Duration,
    pub max_poll_records: i32,
    pub max_poll_interval: Duration,
    pub session_timeout: Duration,
    pub heartbeat_interval: Duration,
    pub auto_offset_reset: AutoOffsetReset,
    pub enable_auto_commit: bool,
    pub isolation_level: IsolationLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub provider: StorageProvider,
    pub region: String,
    pub endpoint: Option<Url>,
    pub bucket: String,
    pub access_key: SecretString,
    pub secret_key: SecretString,
    pub session_token: Option<SecretString>,
    pub use_path_style: bool,
    pub max_upload_size: u64,
    pub multipart_threshold: u64,
    pub multipart_chunk_size: u64,
    pub presign_ttl: Duration,
    pub cdn_url: Option<Url>,
    pub cdn_key_pair_id: Option<String>,
    pub cdn_private_key: Option<SecretString>,
    pub encryption: EncryptionConfig,
    pub lifecycle: LifecycleConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageProvider {
    S3,
    Minio,
    AzureBlob,
    Gcs,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub enabled: bool,
    pub algorithm: EncryptionAlgorithm,
    pub key_id: Option<String>,
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EncryptionAlgorithm {
    Aes256,
    Aes256Gcm,
    ChaCha20Poly1305,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    pub enabled: bool,
    pub transition_days: u32,
    pub expiration_days: u32,
    pub abort_incomplete_multipart_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt: JwtConfig,
    pub oauth2: OAuth2Config,
    pub session: SessionConfig,
    pub mfa: MfaConfig,
    pub password: PasswordConfig,
    pub api_key: ApiKeyConfig,
    pub rate_limit: AuthRateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub issuer: String,
    pub audience: Vec<String>,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub id_token_ttl: Duration,
    pub signing_key: SecretString,
    pub verification_key: Option<SecretString>,
    pub algorithm: JwtAlgorithm,
    pub key_id: Option<String>,
    pub jwks_uri: Option<Url>,
    pub jwks_cache_ttl: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JwtAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
    EdDSA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub enabled: bool,
    pub authorization_server_url: Url,
    pub token_endpoint: Url,
    pub revocation_endpoint: Option<Url>,
    pub introspection_endpoint: Option<Url>,
    pub userinfo_endpoint: Option<Url>,
    pub jwks_uri: Option<Url>,
    pub clients: HashMap<String, OAuth2ClientConfig>,
    pub scopes: HashMap<String, ScopeConfig>,
    pub pkce_required: bool,
    pub device_authorization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2ClientConfig {
    pub client_id: String,
    pub client_secret: SecretString,
    pub redirect_uris: Vec<Url>,
    pub grant_types: Vec<GrantType>,
    pub response_types: Vec<ResponseType>,
    pub scopes: Vec<String>,
    pub token_endpoint_auth_method: TokenEndpointAuthMethod,
    pub require_pkce: bool,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub id_token_ttl: Duration,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeConfig {
    pub description: String,
    pub default: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub ttl: Duration,
    pub idle_timeout: Duration,
    pub absolute_timeout: Duration,
    pub rotating: bool,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    pub domain: Option<String>,
    pub path: String,
    pub store: SessionStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStore {
    Redis,
    Database,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    pub enabled: bool,
    pub required_for_roles: Vec<String>,
    pub totp: TotpConfig,
    pub webauthn: WebAuthnConfig,
    pub backup_codes: BackupCodesConfig,
    pub recovery: RecoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    pub issuer: String,
    pub digits: u32,
    pub period: u64,
    pub algorithm: TotpAlgorithm,
    pub skew: u32,
    pub qr_code_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TotpAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnConfig {
    pub enabled: bool,
    pub relying_party_id: String,
    pub relying_party_name: String,
    pub relying_party_origin: Url,
    pub attestation_conveyance: AttestationConveyance,
    pub authenticator_attachment: AuthenticatorAttachment,
    pub resident_key: ResidentKeyRequirement,
    pub user_verification: UserVerificationRequirement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttestationConveyance {
    None,
    Indirect,
    Direct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticatorAttachment {
    Platform,
    CrossPlatform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResidentKeyRequirement {
    Discouraged,
    Preferred,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserVerificationRequirement {
    Required,
    Preferred,
    Discouraged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodesConfig {
    pub enabled: bool,
    pub count: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryConfig {
    pub enabled: bool,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub token_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    pub min_length: usize,
    pub max_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
    pub forbid_common: bool,
    pub forbid_personal_info: bool,
    pub history_count: usize,
    pub max_age: Option<Duration>,
    pub argon2: Argon2Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Config {
    pub variant: Argon2Variant,
    pub version: u32,
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
    pub hash_length: usize,
    pub salt_length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Argon2Variant {
    Argon2d,
    Argon2i,
    Argon2id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    pub prefix: String,
    pub length: usize,
    pub hash_algorithm: HashAlgorithm,
    pub default_ttl: Option<Duration>,
    pub max_per_user: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
    Argon2id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRateLimitConfig {
    pub login_attempts: RateLimitRule,
    pub register_attempts: RateLimitRule,
    pub password_reset_attempts: RateLimitRule,
    pub mfa_attempts: RateLimitRule,
    pub token_refresh_attempts: RateLimitRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitRule {
    pub max_requests: u32,
    pub window: Duration,
    pub block_duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub tracing: TracingConfig,
    pub metrics: MetricsConfig,
    pub logging: LoggingConfig,
    pub profiling: ProfilingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub service_name: String,
    pub environment: String,
    pub sampler: SamplerConfig,
    pub exporter: ExporterConfig,
    pub propagators: Vec<PropagatorType>,
    pub resource_attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SamplerConfig {
    AlwaysOn,
    AlwaysOff,
    TraceIdRatio(f64),
    ParentBasedTraceIdRatio(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExporterConfig {
    pub endpoint: Url,
    pub protocol: ExporterProtocol,
    pub headers: HashMap<String, String>,
    pub timeout: Duration,
    pub batch: BatchConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExporterProtocol {
    Grpc,
    HttpProtobuf,
    HttpJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub max_queue_size: usize,
    pub max_export_batch_size: usize,
    pub schedule_delay: Duration,
    pub export_timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropagatorType {
    TraceContext,
    Baggage,
    B3,
    B3Single,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub endpoint: Option<Url>,
    pub push_interval: Duration,
    pub namespace: String,
    pub buckets: Vec<f64>,
    pub exemplars: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: LogOutput,
    pub json_fields: HashMap<String, String>,
    pub sampling: SamplingConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Text,
    Pretty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Stdout,
    Stderr,
    File,
    Syslog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingConfig {
    pub enabled: bool,
    pub rate: f64,
    pub max_per_second: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub endpoint: Option<Url>,
    pub interval: Duration,
    pub cpu: bool,
    pub heap: bool,
    pub goroutine: bool,
    pub mutex: bool,
    pub block: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption: EncryptionSettings,
    pub signing: SigningSettings,
    pub headers: SecurityHeaders,
    pub cors: CorsConfig,
    pub rate_limiting: RateLimitingConfig,
    pub waf: WafConfig,
    pub audit: AuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSettings {
    pub algorithm: EncryptionAlgorithm,
    pub key_rotation_days: u32,
    pub key_derivation: KeyDerivation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyDerivation {
    Pbkdf2,
    Argon2id,
    Scrypt,
    Hkdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningSettings {
    pub algorithm: SigningAlgorithm,
    pub key_rotation_days: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SigningAlgorithm {
    Ed25519,
    ES256,
    ES384,
    ES512,
    RS256,
    RS384,
    RS512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeaders {
    pub hsts: HstsConfig,
    pub csp: CspConfig,
    pub referrer_policy: ReferrerPolicy,
    pub permissions_policy: PermissionsPolicy,
    pub cross_origin_embedder_policy: CrossOriginPolicy,
    pub cross_origin_opener_policy: CrossOriginPolicy,
    pub cross_origin_resource_policy: CrossOriginPolicy,
    pub x_frame_options: FrameOptions,
    pub x_content_type_options: bool,
    pub x_xss_protection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HstsConfig {
    pub enabled: bool,
    pub max_age: Duration,
    pub include_subdomains: bool,
    pub preload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CspConfig {
    pub enabled: bool,
    pub policy: String,
    pub report_only: bool,
    pub report_uri: Option<Url>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsPolicy {
    pub enabled: bool,
    pub directives: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CrossOriginPolicy {
    UnsafeNone,
    SameOrigin,
    SameOriginAllowCredentials,
    RequireCorp,
    Credentialless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FrameOptions {
    Deny,
    SameOrigin,
    AllowFrom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub default: RateLimitRule,
    pub rules: HashMap<String, RateLimitRule>,
    pub storage: RateLimitStorage,
    pub key_generator: KeyGenerator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RateLimitStorage {
    Redis,
    Memory,
    Database,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyGenerator {
    Ip,
    User,
    ApiKey,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WafConfig {
    pub enabled: bool,
    pub mode: WafMode,
    pub rules: Vec<WafRule>,
    pub exclusions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WafMode {
    Monitor,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WafRule {
    pub id: String,
    pub name: String,
    pub conditions: Vec<WafCondition>,
    pub action: WafAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WafCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
    pub negation: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WafAction {
    Allow,
    Block,
    Challenge,
    Log,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enabled: bool,
    pub events: Vec<AuditEventType>,
    pub retention_days: u32,
    pub storage: AuditStorage,
    pub encryption: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    ConfigurationChange,
    SecurityEvent,
    SystemEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditStorage {
    Database,
    File,
    S3,
    Elasticsearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub flags: HashMap<String, bool>,
    pub rollout: HashMap<String, RolloutConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutConfig {
    pub percentage: u8,
    pub groups: Vec<String>,
    pub tenants: Vec<TenantId>,
    pub users: Vec<crate::domain::UserId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    pub max_request_size: usize,
    pub max_response_size: usize,
    pub max_concurrent_requests: usize,
    pub max_file_upload_size: u64,
    pub max_pagination_limit: u32,
    pub max_search_results: u32,
    pub max_batch_size: u32,
    pub max_webhook_payload_size: usize,
    pub max_lti_launch_size: usize,
    pub max_scorm_package_size: u64,
    pub max_certificate_size: u64,
    pub max_badge_size: u64,
    pub rate_limit: RateLimitRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsConfig {
    pub lti: LtiConfig,
    pub scorm: ScormConfig,
    pub xapi: XApiConfig,
    pub caliper: CaliperConfig,
    pub oneroster: OneRosterConfig,
    pub qti: QtiConfig,
    pub ai: AiConfig,
    pub email: EmailConfig,
    pub sms: SmsConfig,
    pub push: PushConfig,
    pub webhooks: WebhooksConfig,
    pub search: SearchConfig,
    pub cdn: CdnConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LtiConfig {
    pub enabled: bool,
    pub platform: LtiPlatformConfig,
    pub tool: LtiToolConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LtiPlatformConfig {
    pub issuer: String,
    pub client_id: String,
    pub private_key: SecretString,
    pub key_id: String,
    pub deployment_id: String,
    pub jwks_url: Url,
    pub supported_versions: Vec<String>,
    pub supported_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LtiToolConfig {
    pub enabled: bool,
    pub title: String,
    pub description: String,
    pub target_link_uri: Url,
    pub icon_url: Option<Url>,
    pub custom_parameters: HashMap<String, String>,
    pub privacy_level: PrivacyLevel,
    pub extensions: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyLevel {
    Anonymous,
    NameOnly,
    EmailOnly,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScormConfig {
    pub enabled: bool,
    pub player_url: Url,
    pub max_package_size: u64,
    pub supported_versions: Vec<String>,
    pub sequencing: bool,
    pub navigation: bool,
    pub metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiConfig {
    pub enabled: bool,
    pub lrs_endpoint: Url,
    pub auth_token: SecretString,
    pub actor_inference: bool,
    pub statement_validation: bool,
    pub batch_size: usize,
    pub flush_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaliperConfig {
    pub enabled: bool,
    pub endpoint: Url,
    pub sensor_id: String,
    pub batch_size: usize,
    pub flush_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneRosterConfig {
    pub enabled: bool,
    pub base_url: Url,
    pub client_id: String,
    pub client_secret: SecretString,
    pub sync_interval: Duration,
    pub academic_sessions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QtiConfig {
    pub enabled: bool,
    pub import_enabled: bool,
    pub export_enabled: bool,
    pub supported_versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub providers: HashMap<String, AiProviderConfig>,
    pub default_provider: String,
    pub model_registry: ModelRegistryConfig,
    pub feature_store: FeatureStoreConfig,
    pub inference: InferenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProviderConfig {
    pub type_: AiProviderType,
    pub api_key: SecretString,
    pub base_url: Option<Url>,
    pub organization_id: Option<String>,
    pub models: Vec<String>,
    pub rate_limit: RateLimitRule,
    pub timeout: Duration,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiProviderType {
    OpenAI,
    Anthropic,
    Google,
    Cohere,
    Azure,
    Bedrock,
    VertexAI,
    HuggingFace,
    Ollama,
    VLLM,
    TGI,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryConfig {
    pub enabled: bool,
    pub storage_path: PathBuf,
    pub mlflow_tracking_uri: Option<Url>,
    pub auto_register: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStoreConfig {
    pub enabled: bool,
    pub provider: FeatureStoreProvider,
    pub redis_url: Option<SecretString>,
    pub offline_store: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeatureStoreProvider {
    Feast,
    Redis,
    Database,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub batch_size: usize,
    pub max_concurrent: usize,
    pub timeout: Duration,
    pub cache_ttl: Duration,
    pub fallback_provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub provider: EmailProvider,
    pub from_address: String,
    pub from_name: String,
    pub reply_to: Option<String>,
    pub smtp: SmtpConfig,
    pub templates_path: PathBuf,
    pub rate_limit: RateLimitRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailProvider {
    Smtp,
    SendGrid,
    Mailgun,
    Postmark,
    SES,
    Resend,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub tls: bool,
    pub starttls: bool,
    pub auth: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsConfig {
    pub provider: SmsProvider,
    pub from_number: String,
    pub account_sid: Option<String>,
    pub auth_token: Option<SecretString>,
    pub rate_limit: RateLimitRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmsProvider {
    Twilio,
    Plivo,
    Vonage,
    Telnyx,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushConfig {
    pub provider: PushProvider,
    pub apns: ApnsConfig,
    pub fcm: FcmConfig,
    pub webpush: WebPushConfig,
    pub rate_limit: RateLimitRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PushProvider {
    Apns,
    Fcm,
    WebPush,
    OneSignal,
    Firebase,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApnsConfig {
    pub key_id: String,
    pub team_id: String,
    pub bundle_id: String,
    pub private_key: SecretString,
    pub production: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcmConfig {
    pub project_id: String,
    pub credentials_json: SecretString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebPushConfig {
    pub vapid_public_key: String,
    pub vapid_private_key: SecretString,
    pub vapid_subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhooksConfig {
    pub enabled: bool,
    pub secret: SecretString,
    pub signature_header: String,
    pub timestamp_header: String,
    pub tolerance: Duration,
    pub retry: WebhookRetryConfig,
    pub max_concurrent_deliveries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRetryConfig {
    pub max_attempts: u32,
    pub initial_interval: Duration,
    pub max_interval: Duration,
    pub multiplier: f64,
    pub max_elapsed_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub provider: SearchProvider,
    pub meilisearch: MeilisearchConfig,
    pub typesense: TypesenseConfig,
    pub elasticsearch: ElasticsearchConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchProvider {
    Meilisearch,
    Typesense,
    Elasticsearch,
    Postgres,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeilisearchConfig {
    pub url: Url,
    pub api_key: Option<SecretString>,
    pub index_prefix: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypesenseConfig {
    pub nodes: Vec<TypesenseNode>,
    pub api_key: SecretString,
    pub connection_timeout: Duration,
    pub healthcheck_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypesenseNode {
    pub host: String,
    pub port: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticsearchConfig {
    pub urls: Vec<Url>,
    pub username: Option<String>,
    pub password: Option<SecretString>,
    pub api_key: Option<SecretString>,
    pub ca_path: Option<PathBuf>,
    pub index_prefix: String,
    pub sniff: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnConfig {
    pub enabled: bool,
    pub provider: CdnProvider,
    pub base_url: Url,
    pub signing_key: Option<SecretString>,
    pub key_pair_id: Option<String>,
    pub default_ttl: Duration,
    pub max_ttl: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CdnProvider {
    CloudFront,
    Cloudflare,
    Fastly,
    Akamai,
    Bunny,
    Custom,
}

/// Load settings from files and environment
impl Settings {
    /// Load configuration from default locations
    pub fn load() -> Result<Self> {
        let config = Config::builder()
            .add_source(File::from_str(include_str!("default.toml"), FileFormat::Toml))
            .add_source(File::with_name("config").required(false))
            .add_source(File::with_name(&format!("config.{}", std::env::var("ENVIRONMENT").unwrap_or_default())).required(false))
            .add_source(Environment::with_prefix("APEXOS").separator("__").try_parsing(true))
            .build()?;

        config.try_deserialize().map_err(Into::into)
    }

    /// Load with Figment for more complex scenarios
    pub fn load_with_figment() -> Result<Self> {
        Figment::new()
            .merge(figment::providers::Toml::file("config.toml"))
            .merge(figment::providers::Env::prefixed("APEXOS_").global())
            .extract()
            .map_err(Into::into)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.service.name.is_empty() {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::InternalError,
                "service.name is required",
            ));
        }

        if self.server.http.address.port() == 0 {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::InternalError,
                "server.http.address.port must be > 0",
            ));
        }

        if self.database.url.expose_secret().is_empty() {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::InternalError,
                "database.url is required",
            ));
        }

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            service: ServiceConfig {
                name: "apexos-service".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                environment: Environment::Development,
                instance_id: ulid::Ulid::new().to_string(),
                tags: HashMap::new(),
            },
            server: ServerConfig {
                http: HttpServerConfig {
                    address: "0.0.0.0:8000".parse().unwrap(),
                    workers: None,
                    max_connections: Some(10000),
                    request_timeout: Duration::from_secs(30),
                    keep_alive_timeout: Duration::from_secs(75),
                    body_limit: 10 * 1024 * 1024,
                    tls: None,
                    cors: CorsConfig::default(),
                    compression: true,
                    trusted_proxies: vec!["10.0.0.0/8".to_string(), "172.16.0.0/12".to_string(), "192.168.0.0/16".to_string()],
                },
                grpc: None,
                websocket: None,
                shutdown_timeout: Duration::from_secs(30),
            },
            database: DatabaseConfig {
                url: SecretString::new("postgresql://postgres:postgres@localhost:5432/axel".into()),
                max_connections: 20,
                min_connections: 5,
                connect_timeout: Duration::from_secs(10),
                idle_timeout: Duration::from_secs(600),
                max_lifetime: Duration::from_secs(1800),
                acquire_timeout: Duration::from_secs(10),
                sqlx_offline: false,
                migrations_path: PathBuf::from("migrations"),
                read_replicas: vec![],
                statement_cache_capacity: 100,
            },
            redis: RedisConfig {
                url: SecretString::new("redis://localhost:6379".into()),
                cluster_urls: vec![],
                max_connections: 50,
                connection_timeout: Duration::from_secs(5),
                command_timeout: Duration::from_secs(5),
                pool_size: 10,
                enable_cluster: false,
                enable_sentinel: false,
                sentinel_master: None,
                sentinel_urls: vec![],
                tls: false,
                username: None,
                password: None,
                database: 0,
            },
            nats: NatsConfig {
                urls: vec!["nats://localhost:4222".to_string()],
                username: None,
                password: None,
                token: None,
                nkey: None,
                tls: false,
                tls_config: None,
                max_reconnects: 10,
                reconnect_delay: Duration::from_secs(1),
                ping_interval: Duration::from_secs(30),
                max_pending: 65536,
                jetstream: JetStreamConfig {
                    enabled: true,
                    domain: None,
                    api_prefix: "api".to_string(),
                    stream_config: StreamConfig {
                        name: "events".to_string(),
                        subjects: vec!["events.>".to_string()],
                        retention: RetentionPolicy::Limits,
                        max_consumers: -1,
                        max_msgs: -1,
                        max_bytes: -1,
                        max_age: Duration::from_secs(86400 * 7),
                        max_msg_size: 1024 * 1024,
                        storage: StorageType::File,
                        replicas: 1,
                        no_ack: false,
                        duplicate_window: Duration::from_secs(120),
                        placement: None,
                        mirror: None,
                        sources: vec![],
                        allow_rollup: false,
                        deny_delete: false,
                        deny_purge: false,
                    },
                    consumer_config: ConsumerConfig {
                        durable_name: None,
                        ack_policy: AckPolicy::Explicit,
                        ack_wait: Duration::from_secs(30),
                        max_deliver: 3,
                        filter_subject: None,
                        replay_policy: ReplayPolicy::Instant,
                        rate_limit: 0,
                        max_ack_pending: 1000,
                        flow_control: false,
                        heartbeat: Duration::from_secs(30),
                        headers_only: false,
                    },
                },
            },
            kafka: KafkaConfig {
                bootstrap_servers: "localhost:9092".to_string(),
                client_id: "apexos".to_string(),
                group_id: "apexos".to_string(),
                security_protocol: SecurityProtocol::Plaintext,
                sasl_mechanism: None,
                sasl_username: None,
                sasl_password: None,
                ssl_ca_path: None,
                ssl_cert_path: None,
                ssl_key_path: None,
                enable_auto_commit: true,
                auto_offset_reset: AutoOffsetReset::Earliest,
                session_timeout: Duration::from_secs(30),
                heartbeat_interval: Duration::from_secs(10),
                max_poll_records: 500,
                max_poll_interval: Duration::from_secs(300),
                fetch_min_bytes: 1,
                fetch_max_wait: Duration::from_millis(500),
                producer: ProducerConfig {
                    acks: Acks::All,
                    retries: 3,
                    enable_idempotence: true,
                    max_in_flight: 5,
                    compression: CompressionType::Zstd,
                    linger_ms: 5,
                    batch_size: 16384,
                    buffer_memory: 33554432,
                    delivery_timeout: Duration::from_secs(120),
                    request_timeout: Duration::from_secs(30),
                },
                consumer: KafkaConsumerConfig {
                    fetch_min_bytes: 1,
                    fetch_max_wait: Duration::from_millis(500),
                    max_poll_records: 500,
                    max_poll_interval: Duration::from_secs(300),
                    session_timeout: Duration::from_secs(30),
                    heartbeat_interval: Duration::from_secs(10),
                    auto_offset_reset: AutoOffsetReset::Earliest,
                    enable_auto_commit: true,
                    isolation_level: IsolationLevel::ReadCommitted,
                },
            },
            storage: StorageConfig {
                provider: StorageProvider::Minio,
                region: "us-east-1".to_string(),
                endpoint: Some("http://localhost:9000".parse().unwrap()),
                bucket: "axel".to_string(),
                access_key: SecretString::new("minioadmin".into()),
                secret_key: SecretString::new("minioadmin".into()),
                session_token: None,
                use_path_style: true,
                max_upload_size: 5 * 1024 * 1024 * 1024,
                multipart_threshold: 100 * 1024 * 1024,
                multipart_chunk_size: 50 * 1024 * 1024,
                presign_ttl: Duration::from_secs(3600),
                cdn_url: None,
                cdn_key_pair_id: None,
                cdn_private_key: None,
                encryption: EncryptionConfig {
                    enabled: false,
                    algorithm: EncryptionAlgorithm::Aes256,
                    key_id: None,
                    kms_key_id: None,
                },
                lifecycle: LifecycleConfig {
                    enabled: false,
                    transition_days: 30,
                    expiration_days: 365,
                    abort_incomplete_multipart_days: 7,
                },
            },
            auth: AuthConfig {
                jwt: JwtConfig {
                    issuer: "apexos".to_string(),
                    audience: vec!["axel".to_string()],
                    access_token_ttl: Duration::from_secs(900),
                    refresh_token_ttl: Duration::from_secs(86400 * 30),
                    id_token_ttl: Duration::from_secs(3600),
                    signing_key: SecretString::new("dev-secret-change-in-production".into()),
                    verification_key: None,
                    algorithm: JwtAlgorithm::HS256,
                    key_id: None,
                    jwks_uri: None,
                    jwks_cache_ttl: Duration::from_secs(3600),
                },
                oauth2: OAuth2Config {
                    enabled: false,
                    authorization_server_url: "https://auth.axel.local".parse().unwrap(),
                    token_endpoint: "https://auth.axel.local/oauth/token".parse().unwrap(),
                    revocation_endpoint: None,
                    introspection_endpoint: None,
                    userinfo_endpoint: None,
                    jwks_uri: None,
                    clients: HashMap::new(),
                    scopes: HashMap::new(),
                    pkce_required: true,
                    device_authorization: false,
                },
                session: SessionConfig {
                    ttl: Duration::from_secs(86400 * 30),
                    idle_timeout: Duration::from_secs(3600),
                    absolute_timeout: Duration::from_secs(86400 * 30),
                    rotating: true,
                    secure: true,
                    http_only: true,
                    same_site: SameSite::Lax,
                    domain: None,
                    path: "/".to_string(),
                    store: SessionStore::Redis,
                },
                mfa: MfaConfig {
                    enabled: true,
                    required_for_roles: vec!["admin".to_string(), "owner".to_string()],
                    totp: TotpConfig {
                        issuer: "Axel LMS".to_string(),
                        digits: 6,
                        period: 30,
                        algorithm: TotpAlgorithm::SHA1,
                        skew: 1,
                        qr_code_size: 256,
                    },
                    webauthn: WebAuthnConfig {
                        enabled: true,
                        relying_party_id: "axel.local".to_string(),
                        relying_party_name: "Axel LMS".to_string(),
                        relying_party_origin: "https://axel.local".parse().unwrap(),
                        attestation_conveyance: AttestationConveyance::None,
                        authenticator_attachment: AuthenticatorAttachment::CrossPlatform,
                        resident_key: ResidentKeyRequirement::Preferred,
                        user_verification: UserVerificationRequirement::Preferred,
                    },
                    backup_codes: BackupCodesConfig {
                        enabled: true,
                        count: 10,
                        length: 8,
                    },
                    recovery: RecoveryConfig {
                        enabled: true,
                        email_enabled: true,
                        sms_enabled: false,
                        token_ttl: Duration::from_secs(3600),
                    },
                },
                password: PasswordConfig {
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
                        variant: Argon2Variant::Argon2id,
                        version: 19,
                        memory_cost: 65536,
                        time_cost: 3,
                        parallelism: 4,
                        hash_length: 32,
                        salt_length: 16,
                    },
                },
                api_key: ApiKeyConfig {
                    prefix: "axl".to_string(),
                    length: 32,
                    hash_algorithm: HashAlgorithm::Argon2id,
                    default_ttl: Some(Duration::from_secs(86400 * 365)),
                    max_per_user: 10,
                },
                rate_limit: AuthRateLimitConfig {
                    login_attempts: RateLimitRule {
                        max_requests: 5,
                        window: Duration::from_secs(300),
                        block_duration: Some(Duration::from_secs(900)),
                    },
                    register_attempts: RateLimitRule {
                        max_requests: 3,
                        window: Duration::from_secs(3600),
                        block_duration: Some(Duration::from_secs(3600)),
                    },
                    password_reset_attempts: RateLimitRule {
                        max_requests: 2,
                        window: Duration::from_secs(3600),
                        block_duration: Some(Duration::from_secs(3600)),
                    },
                    mfa_attempts: RateLimitRule {
                        max_requests: 5,
                        window: Duration::from_secs(300),
                        block_duration: Some(Duration::from_secs(900)),
                    },
                    token_refresh_attempts: RateLimitRule {
                        max_requests: 10,
                        window: Duration::from_secs(60),
                        block_duration: None,
                    },
                },
            },
            observability: ObservabilityConfig {
                tracing: TracingConfig {
                    enabled: true,
                    service_name: "apexos".to_string(),
                    environment: "development".to_string(),
                    sampler: SamplerConfig::AlwaysOn,
                    exporter: ExporterConfig {
                        endpoint: "http://localhost:4317".parse().unwrap(),
                        protocol: ExporterProtocol::Grpc,
                        headers: HashMap::new(),
                        timeout: Duration::from_secs(10),
                        batch: BatchConfig {
                            max_queue_size: 2048,
                            max_export_batch_size: 512,
                            schedule_delay: Duration::from_secs(5),
                            export_timeout: Duration::from_secs(30),
                        },
                    },
                    propagators: vec![PropagatorType::TraceContext, PropagatorType::Baggage],
                    resource_attributes: HashMap::new(),
                },
                metrics: MetricsConfig {
                    enabled: true,
                    endpoint: None,
                    push_interval: Duration::from_secs(10),
                    namespace: "apexos".to_string(),
                    buckets: vec![
                        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                    ],
                    exemplars: true,
                },
                logging: LoggingConfig {
                    level: LogLevel::Info,
                    format: LogFormat::Json,
                    output: LogOutput::Stdout,
                    json_fields: HashMap::new(),
                    sampling: SamplingConfig {
                        enabled: false,
                        rate: 1.0,
                        max_per_second: 1000,
                    },
                },
                profiling: ProfilingConfig {
                    enabled: false,
                    endpoint: None,
                    interval: Duration::from_secs(60),
                    cpu: true,
                    heap: true,
                    goroutine: false,
                    mutex: true,
                    block: true,
                },
            },
            security: SecurityConfig {
                encryption: EncryptionSettings {
                    algorithm: EncryptionAlgorithm::Aes256Gcm,
                    key_rotation_days: 90,
                    key_derivation: KeyDerivation::Argon2id,
                },
                signing: SigningSettings {
                    algorithm: SigningAlgorithm::Ed25519,
                    key_rotation_days: 90,
                },
                headers: SecurityHeaders {
                    hsts: HstsConfig {
                        enabled: true,
                        max_age: Duration::from_secs(31536000),
                        include_subdomains: true,
                        preload: true,
                    },
                    csp: CspConfig {
                        enabled: true,
                        policy: "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'".to_string(),
                        report_only: false,
                        report_uri: None,
                    },
                    referrer_policy: ReferrerPolicy::StrictOriginWhenCrossOrigin,
                    permissions_policy: PermissionsPolicy {
                        enabled: true,
                        directives: HashMap::from([
                            ("camera".to_string(), vec!["'none'".to_string()]),
                            ("microphone".to_string(), vec!["'none'".to_string()]),
                            ("geolocation".to_string(), vec!["'none'".to_string()]),
                        ]),
                    },
                    cross_origin_embedder_policy: CrossOriginPolicy::RequireCorp,
                    cross_origin_opener_policy: CrossOriginPolicy::SameOrigin,
                    cross_origin_resource_policy: CrossOriginPolicy::SameOrigin,
                    x_frame_options: FrameOptions::Deny,
                    x_content_type_options: true,
                    x_xss_protection: true,
                },
                cors: CorsConfig::default(),
                rate_limiting: RateLimitingConfig {
                    enabled: true,
                    default: RateLimitRule {
                        max_requests: 100,
                        window: Duration::from_secs(60),
                        block_duration: None,
                    },
                    rules: HashMap::new(),
                    storage: RateLimitStorage::Redis,
                    key_generator: KeyGenerator::Ip,
                },
                waf: WafConfig {
                    enabled: true,
                    mode: WafMode::Monitor,
                    rules: vec![],
                    exclusions: vec![],
                },
                audit: AuditConfig {
                    enabled: true,
                    events: vec![
                        AuditEventType::Authentication,
                        AuditEventType::Authorization,
                        AuditEventType::DataAccess,
                        AuditEventType::DataModification,
                        AuditEventType::SecurityEvent,
                    ],
                    retention_days: 2555,
                    storage: AuditStorage::Database,
                    encryption: true,
                },
            },
            features: FeatureFlags {
                flags: HashMap::new(),
                rollout: HashMap::new(),
            },
            limits: LimitsConfig {
                max_request_size: 10 * 1024 * 1024,
                max_response_size: 50 * 1024 * 1024,
                max_concurrent_requests: 10000,
                max_file_upload_size: 5 * 1024 * 1024 * 1024,
                max_pagination_limit: 1000,
                max_search_results: 10000,
                max_batch_size: 1000,
                max_webhook_payload_size: 1024 * 1024,
                max_lti_launch_size: 1024 * 1024,
                max_scorm_package_size: 2 * 1024 * 1024 * 1024,
                max_certificate_size: 10 * 1024 * 1024,
                max_badge_size: 10 * 1024 * 1024,
                rate_limit: RateLimitRule {
                    max_requests: 1000,
                    window: Duration::from_secs(60),
                    block_duration: None,
                },
            },
            integrations: IntegrationsConfig {
                lti: LtiConfig {
                    enabled: true,
                    platform: LtiPlatformConfig {
                        issuer: "https://axel.local".to_string(),
                        client_id: "axel-platform".to_string(),
                        private_key: SecretString::new("".into()),
                        key_id: "axel-key-1".to_string(),
                        deployment_id: "axel-deployment".to_string(),
                        jwks_url: "https://axel.local/.well-known/jwks.json".parse().unwrap(),
                        supported_versions: vec!["1.3".to_string()],
                        supported_extensions: vec!["assignment", "names", "deep-linking", "membership", "scores", "proctoring"].into_iter().map(String::from).collect(),
                    },
                    tool: LtiToolConfig {
                        enabled: true,
                        title: "Axel LMS Tool".to_string(),
                        description: "Axel LMS LTI Tool".to_string(),
                        target_link_uri: "https://axel.local/lti/launch".parse().unwrap(),
                        icon_url: None,
                        custom_parameters: HashMap::new(),
                        privacy_level: PrivacyLevel::Full,
                        extensions: HashMap::new(),
                    },
                },
                scorm: ScormConfig {
                    enabled: true,
                    player_url: "https://axel.local/scorm/player".parse().unwrap(),
                    max_package_size: 2 * 1024 * 1024 * 1024,
                    supported_versions: vec!["2004 4th Edition".to_string(), "1.2".to_string()],
                    sequencing: true,
                    navigation: true,
                    metadata: true,
                },
                xapi: XApiConfig {
                    enabled: true,
                    lrs_endpoint: "https://axel.local/xapi".parse().unwrap(),
                    auth_token: SecretString::new("".into()),
                    actor_inference: true,
                    statement_validation: true,
                    batch_size: 100,
                    flush_interval: Duration::from_secs(30),
                },
                caliper: CaliperConfig {
                    enabled: true,
                    endpoint: "https://axel.local/caliper".parse().unwrap(),
                    sensor_id: "axel-lms".to_string(),
                    batch_size: 100,
                    flush_interval: Duration::from_secs(30),
                },
                oneroster: OneRosterConfig {
                    enabled: false,
                    base_url: "https://sis.example.com/ims/oneroster/v1p2".parse().unwrap(),
                    client_id: "".to_string(),
                    client_secret: SecretString::new("".into()),
                    sync_interval: Duration::from_secs(3600),
                    academic_sessions: vec![],
                },
                qti: QtiConfig {
                    enabled: true,
                    import_enabled: true,
                    export_enabled: true,
                    supported_versions: vec!["3.0".to_string(), "2.2".to_string(), "2.1".to_string()],
                },
                ai: AiConfig {
                    enabled: false,
                    providers: HashMap::new(),
                    default_provider: "openai".to_string(),
                    model_registry: ModelRegistryConfig {
                        enabled: true,
                        storage_path: PathBuf::from("/var/lib/axel/models"),
                        mlflow_tracking_uri: None,
                        auto_register: false,
                    },
                    feature_store: FeatureStoreConfig {
                        enabled: false,
                        provider: FeatureStoreProvider::Redis,
                        redis_url: None,
                        offline_store: None,
                    },
                    inference: InferenceConfig {
                        batch_size: 32,
                        max_concurrent: 10,
                        timeout: Duration::from_secs(30),
                        cache_ttl: Duration::from_secs(3600),
                        fallback_provider: None,
                    },
                },
                email: EmailConfig {
                    provider: EmailProvider::Smtp,
                    from_address: "noreply@axel.local".to_string(),
                    from_name: "Axel LMS".to_string(),
                    reply_to: None,
                    smtp: SmtpConfig {
                        host: "localhost".to_string(),
                        port: 1025,
                        username: "".to_string(),
                        password: SecretString::new("".into()),
                        tls: false,
                        starttls: false,
                        auth: false,
                    },
                    templates_path: PathBuf::from("templates/email"),
                    rate_limit: RateLimitRule {
                        max_requests: 100,
                        window: Duration::from_secs(60),
                        block_duration: None,
                    },
                },
                sms: SmsConfig {
                    provider: SmsProvider::Twilio,
                    from_number: "".to_string(),
                    account_sid: None,
                    auth_token: None,
                    rate_limit: RateLimitRule {
                        max_requests: 10,
                        window: Duration::from_secs(60),
                        block_duration: None,
                    },
                },
                push: PushConfig {
                    provider: PushProvider::WebPush,
                    apns: ApnsConfig {
                        key_id: "".to_string(),
                        team_id: "".to_string(),
                        bundle_id: "".to_string(),
                        private_key: SecretString::new("".into()),
                        production: false,
                    },
                    fcm: FcmConfig {
                        project_id: "".to_string(),
                        credentials_json: SecretString::new("".into()),
                    },
                    webpush: WebPushConfig {
                        vapid_public_key: "".to_string(),
                        vapid_private_key: SecretString::new("".into()),
                        vapid_subject: "mailto:admin@axel.local".to_string(),
                    },
                    rate_limit: RateLimitRule {
                        max_requests: 100,
                        window: Duration::from_secs(60),
                        block_duration: None,
                    },
                },
                webhooks: WebhooksConfig {
                    enabled: true,
                    secret: SecretString::new("webhook-secret".into()),
                    signature_header: "X-Axel-Signature".to_string(),
                    timestamp_header: "X-Axel-Timestamp".to_string(),
                    tolerance: Duration::from_secs(300),
                    retry: WebhookRetryConfig {
                        max_attempts: 5,
                        initial_interval: Duration::from_secs(5),
                        max_interval: Duration::from_secs(3600),
                        multiplier: 2.0,
                        max_elapsed_time: Duration::from_secs(86400),
                    },
                    max_concurrent_deliveries: 100,
                },
                search: SearchConfig {
                    provider: SearchProvider::Meilisearch,
                    meilisearch: MeilisearchConfig {
                        url: "http://localhost:7700".parse().unwrap(),
                        api_key: None,
                        index_prefix: "axel_".to_string(),
                        timeout: Duration::from_secs(10),
                    },
                    typesense: TypesenseConfig {
                        nodes: vec![TypesenseNode {
                            host: "localhost".to_string(),
                            port: 8108,
                            protocol: "http".to_string(),
                        }],
                        api_key: SecretString::new("".into()),
                        connection_timeout: Duration::from_secs(5),
                        healthcheck_interval: Duration::from_secs(30),
                    },
                    elasticsearch: ElasticsearchConfig {
                        urls: vec!["http://localhost:9200".parse().unwrap()],
                        username: None,
                        password: None,
                        api_key: None,
                        ca_path: None,
                        index_prefix: "axel_".to_string(),
                        sniff: true,
                    },
                },
                cdn: CdnConfig {
                    enabled: false,
                    provider: CdnProvider::CloudFront,
                    base_url: "https://cdn.axel.local".parse().unwrap(),
                    signing_key: None,
                    key_pair_id: None,
                    default_ttl: Duration::from_secs(86400),
                    max_ttl: Duration::from_secs(31536000),
                },
            },
            extra: HashMap::new(),
        }
    }
}

/// Trait for loading service-specific configuration
pub trait ServiceConfigExt {
    fn for_service(service_name: &str) -> Result<Self>
    where
        Self: Sized;
}

impl ServiceConfigExt for Settings {
    fn for_service(service_name: &str) -> Result<Self> {
        let mut settings = Self::load()?;
        settings.service.name = service_name.to_string();
        settings.validate()?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_valid() {
        let settings = Settings::default();
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn cors_defaults() {
        let cors = CorsConfig::default();
        assert!(cors.allowed_origins.contains(&"*".to_string()));
        assert!(cors.allow_credentials);
    }
}