//! Event System
//!
//! CloudEvents-compatible event definitions for inter-service communication
//! with NATS JetStream and Apache Kafka.

use crate::domain::*;
use crate::errors::Result;
use crate::utils::{generate_id, generate_ulid, now_millis};
use async_nats::jetstream;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use ulid::Ulid;
use uuid::Uuid;

/// CloudEvents spec version
pub const CLOUDEVENTS_SPEC_VERSION: &str = "1.0";

/// Event metadata following CloudEvents specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Unique event identifier
    #[serde(rename = "id")]
    pub event_id: EventId,

    /// Event source (service name)
    #[serde(rename = "source")]
    pub source: String,

    /// Event type (e.g., "com.apexos.user.created")
    #[serde(rename = "type")]
    pub event_type: String,

    /// CloudEvents spec version
    #[serde(rename = "specversion")]
    pub spec_version: String,

    /// Timestamp of event occurrence
    #[serde(rename = "time", with = "crate::utils::serde_timestamp")]
    pub timestamp: Timestamp,

    /// Correlation ID for request tracing
    #[serde(rename = "correlationid", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<CorrelationId>,

    /// Causation ID for event chaining
    #[serde(rename = "causationid", skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<CausationId>,

    /// Tenant context
    #[serde(rename = "tenantid", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<TenantId>,

    /// Organization context
    #[serde(rename = "organizationid", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<OrganizationId>,

    /// User who triggered the event
    #[serde(rename = "userid", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    /// Additional attributes
    #[serde(flatten)]
    pub attributes: HashMap<String, serde_json::Value>,
}

impl EventMetadata {
    pub fn new(source: impl Into<String>, event_type: impl Into<String>) -> Self {
        Self {
            event_id: EventId::new(),
            source: source.into(),
            event_type: event_type.into(),
            spec_version: CLOUDEVENTS_SPEC_VERSION.to_string(),
            timestamp: Timestamp::now(),
            correlation_id: None,
            causation_id: None,
            tenant_id: None,
            organization_id: None,
            user_id: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation_id(mut self, causation_id: CausationId) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_tenant(mut self, tenant_id: TenantId) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    pub fn with_organization(mut self, organization_id: OrganizationId) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn with_user(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }

    /// Get NATS subject for this event
    pub fn nats_subject(&self) -> String {
        format!("events.{}.{}", self.source.replace('.', "_"), self.event_type.replace('.', "_"))
    }

    /// Get Kafka topic for this event
    pub fn kafka_topic(&self) -> String {
        format!("events.{}", self.source.replace('.', "-"))
    }
}

impl fmt::Display for EventMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.event_type, self.event_id)
    }
}

/// Event envelope containing metadata and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    #[serde(flatten)]
    pub metadata: EventMetadata,
    #[serde(rename = "datacontenttype")]
    pub data_content_type: String,
    #[serde(rename = "data")]
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(source: impl Into<String>, event_type: impl Into<String>, payload: T) -> Self
    where
        T: Serialize,
    {
        Self {
            metadata: EventMetadata::new(source, event_type),
            data_content_type: "application/json".to_string(),
            payload,
        }
    }

    pub fn with_metadata(mut self, metadata: EventMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.data_content_type = content_type.into();
        self
    }
}

/// Core event trait for domain events
pub trait Event: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static {
    const EVENT_TYPE: &'static str;
    const SOURCE: &'static str;

    fn metadata(&self) -> &EventMetadata;
    fn metadata_mut(&mut self) -> &mut EventMetadata;

    fn event_id(&self) -> EventId {
        self.metadata().event_id
    }

    fn timestamp(&self) -> Timestamp {
        self.metadata().timestamp
    }

    fn correlation_id(&self) -> Option<CorrelationId> {
        self.metadata().correlation_id
    }

    fn causation_id(&self) -> Option<CausationId> {
        self.metadata().causation_id
    }

    fn tenant_id(&self) -> Option<TenantId> {
        self.metadata().tenant_id
    }

    fn to_envelope(&self) -> EventEnvelope<&Self>
    where
        Self: Sized,
    {
        EventEnvelope {
            metadata: self.metadata().clone(),
            data_content_type: "application/json".to_string(),
            payload: self,
        }
    }

    fn into_envelope(self) -> EventEnvelope<Self>
    where
        Self: Sized,
    {
        EventEnvelope {
            metadata: self.metadata().clone(),
            data_content_type: "application/json".to_string(),
            payload: self,
        }
    }
}

/// Macro to implement Event trait for a struct
macro_rules! impl_event {
    ($type:ty, $event_type:expr, $source:expr) => {
        impl Event for $type {
            const EVENT_TYPE: &'static str = $event_type;
            const SOURCE: &'static str = $source;

            fn metadata(&self) -> &EventMetadata {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut EventMetadata {
                &mut self.metadata
            }
        }
    };
}

/// Event publisher trait
#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish<T: Event>(&self, event: T) -> Result<()>;
    async fn publish_batch<T: Event>(&self, events: Vec<T>) -> Result<()>;
    async fn publish_envelope(&self, envelope: EventEnvelope<serde_json::Value>) -> Result<()>;
}

/// Event subscriber trait
#[async_trait::async_trait]
pub trait EventSubscriber: Send + Sync {
    type Event: Event;

    async fn subscribe(&self, handler: Arc<dyn EventHandler<Self::Event>>) -> Result<Subscription>;
    async fn unsubscribe(&self, subscription: Subscription) -> Result<()>;
}

/// Event handler trait
#[async_trait::async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle(&self, event: E) -> Result<()>;
    async fn handle_batch(&self, events: Vec<E>) -> Result<()>;
}

/// Subscription handle
#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: Ulid,
    pub subject: String,
    pub queue_group: Option<String>,
}

/// Message bus for publishing and subscribing to events
#[derive(Clone)]
pub struct MessageBus {
    nats: Option<Arc<NatsClient>>,
    kafka: Option<Arc<KafkaClient>>,
    default_publisher: PublisherType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublisherType {
    Nats,
    Kafka,
    Both,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            nats: None,
            kafka: None,
            default_publisher: PublisherType::Nats,
        }
    }

    pub fn with_nats(mut self, client: NatsClient) -> Self {
        self.nats = Some(Arc::new(client));
        self.default_publisher = PublisherType::Nats;
        self
    }

    pub fn with_kafka(mut self, client: KafkaClient) -> Self {
        self.kafka = Some(Arc::new(client));
        if self.nats.is_none() {
            self.default_publisher = PublisherType::Kafka;
        }
        self
    }

    pub fn with_both(mut self, nats: NatsClient, kafka: KafkaClient) -> Self {
        self.nats = Some(Arc::new(nats));
        self.kafka = Some(Arc::new(kafka));
        self.default_publisher = PublisherType::Both;
        self
    }

    pub async fn publish<T: Event>(&self, event: T) -> Result<()> {
        let envelope = event.into_envelope();
        let value = serde_json::to_value(&envelope)?;

        match self.default_publisher {
            PublisherType::Nats => {
                if let Some(nats) = &self.nats {
                    nats.publish(envelope.metadata.nats_subject(), value).await?;
                }
            }
            PublisherType::Kafka => {
                if let Some(kafka) = &self.kafka {
                    kafka.publish(envelope.metadata.kafka_topic(), value).await?;
                }
            }
            PublisherType::Both => {
                if let Some(nats) = &self.nats {
                    nats.publish(envelope.metadata.nats_subject(), value.clone()).await?;
                }
                if let Some(kafka) = &self.kafka {
                    kafka.publish(envelope.metadata.kafka_topic(), value).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn publish_envelope(&self, envelope: EventEnvelope<serde_json::Value>) -> Result<()> {
        let value = serde_json::to_value(&envelope)?;

        match self.default_publisher {
            PublisherType::Nats => {
                if let Some(nats) = &self.nats {
                    nats.publish(envelope.metadata.nats_subject(), value).await?;
                }
            }
            PublisherType::Kafka => {
                if let Some(kafka) = &self.kafka {
                    kafka.publish(envelope.metadata.kafka_topic(), value).await?;
                }
            }
            PublisherType::Both => {
                if let Some(nats) = &self.nats {
                    nats.publish(envelope.metadata.nats_subject(), value.clone()).await?;
                }
                if let Some(kafka) = &self.kafka {
                    kafka.publish(envelope.metadata.kafka_topic(), value).await?;
                }
            }
        }
        Ok(())
    }
}

/// NATS JetStream client
#[derive(Clone)]
pub struct NatsClient {
    pub client: async_nats::Client,
    pub jetstream: Option<jetstream::Context>,
}

impl NatsClient {
    pub async fn connect(settings: &crate::config::NatsConfig) -> Result<Self> {
        let mut options = async_nats::ConnectOptions::new()
            .max_reconnects(settings.max_reconnects)
            .reconnect_delay(settings.reconnect_delay)
            .ping_interval(settings.ping_interval);

        if let (Some(user), Some(pass)) = (&settings.username, &settings.password) {
            options = options.credentials(user, pass.expose_secret());
        }
        if let Some(token) = &settings.token {
            options = options.token(token.expose_secret());
        }
        if let Some(nkey) = &settings.nkey {
            options = options.nkey(nkey.expose_secret());
        }
        if settings.tls {
            options = options.tls(async_nats::tls::default_client_config()?);
        }

        let client = options.connect(settings.urls.first().unwrap()).await?;

        let jetstream = if settings.jetstream.enabled {
            Some(jetstream::new(client.clone()))
        } else {
            None
        };

        Ok(Self { client, jetstream })
    }

    pub async fn publish(&self, subject: String, payload: serde_json::Value) -> Result<()> {
        let data = serde_json::to_vec(&payload)?;
        self.client.publish(subject, data.into()).await?;
        Ok(())
    }

    pub async fn publish_with_headers(&self, subject: String, headers: HashMap<String, String>, payload: serde_json::Value) -> Result<()> {
        let mut message = async_nats::Message::builder().payload(serde_json::to_vec(&payload)?.into());
        for (k, v) in headers {
            message = message.header(k, v);
        }
        self.client.publish(subject, message.build()).await?;
        Ok(())
    }

    pub async fn request(&self, subject: String, payload: serde_json::Value, timeout: Duration) -> Result<serde_json::Value> {
        let data = serde_json::to_vec(&payload)?;
        let response = self.client.request(subject, data.into()).await?;
        let value = serde_json::from_slice(&response.payload)?;
        Ok(value)
    }

    pub async fn subscribe(&self, subject: &str, queue_group: Option<&str>) -> Result<async_nats::Subscriber> {
        let subscriber = if let Some(qg) = queue_group {
            self.client.queue_subscribe(subject.to_string(), qg.to_string()).await?
        } else {
            self.client.subscribe(subject.to_string()).await?
        };
        Ok(subscriber)
    }

    pub async fn create_stream(&self, config: &crate::config::StreamConfig) -> Result<jetstream::Stream> {
        let js = self.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(crate::errors::ErrorCode::InternalError, "JetStream not enabled"))?;
        
        let stream_config = jetstream::stream::Config {
            name: config.name.clone(),
            subjects: config.subjects.clone(),
            retention: match config.retention {
                crate::config::RetentionPolicy::Limits => jetstream::stream::RetentionPolicy::Limits,
                crate::config::RetentionPolicy::Interest => jetstream::stream::RetentionPolicy::Interest,
                crate::config::RetentionPolicy::WorkQueue => jetstream::stream::RetentionPolicy::WorkQueue,
            },
            max_consumers: config.max_consumers,
            max_msgs: config.max_msgs,
            max_bytes: config.max_bytes,
            max_age: config.max_age,
            max_msg_size: config.max_msg_size,
            storage: match config.storage {
                crate::config::StorageType::File => jetstream::stream::StorageType::File,
                crate::config::StorageType::Memory => jetstream::stream::StorageType::Memory,
            },
            replicas: config.replicas,
            no_ack: config.no_ack,
            duplicate_window: config.duplicate_window,
            placement: config.placement.as_ref().map(|p| jetstream::stream::Placement {
                cluster: p.cluster.clone(),
                tags: p.tags.clone(),
            }),
            mirror: config.mirror.as_ref().map(|m| jetstream::stream::Mirror {
                name: m.name.clone(),
                external: jetstream::stream::ExternalStream {
                    api_prefix: m.external.api_prefix.clone(),
                    urls: m.external.urls.clone(),
                },
            }),
            sources: config.sources.iter().map(|s| jetstream::stream::Source {
                name: s.name.clone(),
                external: jetstream::stream::ExternalStream {
                    api_prefix: s.external.api_prefix.clone(),
                    urls: s.external.urls.clone(),
                },
                start_seq: s.start_seq,
                start_time: s.start_time.clone(),
                filter_subject: s.filter_subject.clone(),
            }).collect(),
            allow_rollup: config.allow_rollup,
            deny_delete: config.deny_delete,
            deny_purge: config.deny_purge,
        };

        Ok(js.create_stream(stream_config).await?)
    }

    pub async fn create_consumer(&self, stream: &str, config: &crate::config::ConsumerConfig) -> Result<jetstream::Consumer> {
        let js = self.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(crate::errors::ErrorCode::InternalError, "JetStream not enabled"))?;
        
        let consumer_config = jetstream::consumer::Config {
            durable_name: config.durable_name.clone(),
            ack_policy: match config.ack_policy {
                crate::config::AckPolicy::None => jetstream::consumer::AckPolicy::None,
                crate::config::AckPolicy::All => jetstream::consumer::AckPolicy::All,
                crate::config::AckPolicy::Explicit => jetstream::consumer::AckPolicy::Explicit,
            },
            ack_wait: config.ack_wait,
            max_deliver: config.max_deliver,
            filter_subject: config.filter_subject.clone(),
            replay_policy: match config.replay_policy {
                crate::config::ReplayPolicy::Instant => jetstream::consumer::ReplayPolicy::Instant,
                crate::config::ReplayPolicy::Original => jetstream::consumer::ReplayPolicy::Original,
                crate::config::ReplayPolicy::New => jetstream::consumer::ReplayPolicy::New,
            },
            rate_limit: config.rate_limit,
            max_ack_pending: config.max_ack_pending,
            flow_control: config.flow_control,
            heartbeat: config.heartbeat,
            headers_only: config.headers_only,
            ..Default::default()
        };

        Ok(js.create_consumer_on_stream(stream, consumer_config).await?)
    }
}

/// Kafka client
#[derive(Clone)]
pub struct KafkaClient {
    pub producer: rdkafka::producer::FutureProducer,
    pub consumer: rdkafka::consumer::StreamConsumer,
}

impl KafkaClient {
    pub async fn connect(settings: &crate::config::KafkaConfig) -> Result<Self> {
        let producer: rdkafka::producer::FutureProducer = rdkafka::ClientConfig::new()
            .set("bootstrap.servers", &settings.bootstrap_servers)
            .set("client.id", &settings.client_id)
            .set("message.timeout.ms", settings.producer.delivery_timeout.as_millis().to_string())
            .set("request.timeout.ms", settings.producer.request_timeout.as_millis().to_string())
            .set("enable.idempotence", settings.producer.enable_idempotence.to_string())
            .set("max.in.flight.requests.per.connection", settings.producer.max_in_flight.to_string())
            .set("compression.type", format!("{:?}", settings.producer.compression).to_lowercase())
            .set("linger.ms", settings.producer.linger_ms.to_string())
            .set("batch.size", settings.producer.batch_size.to_string())
            .set("buffer.memory", settings.producer.buffer_memory.to_string())
            .set("retries", settings.producer.retries.to_string())
            .set("acks", format!("{:?}", settings.producer.acks).to_lowercase())
            .create()?;

        let consumer: rdkafka::consumer::StreamConsumer = rdkafka::ClientConfig::new()
            .set("bootstrap.servers", &settings.bootstrap_servers)
            .set("client.id", &settings.client_id)
            .set("group.id", &settings.group_id)
            .set("enable.auto.commit", settings.consumer.enable_auto_commit.to_string())
            .set("auto.offset.reset", format!("{:?}", settings.consumer.auto_offset_reset).to_lowercase())
            .set("session.timeout.ms", settings.consumer.session_timeout.as_millis().to_string())
            .set("heartbeat.interval.ms", settings.consumer.heartbeat_interval.as_millis().to_string())
            .set("max.poll.records", settings.consumer.max_poll_records.to_string())
            .set("max.poll.interval.ms", settings.consumer.max_poll_interval.as_millis().to_string())
            .set("fetch.min.bytes", settings.consumer.fetch_min_bytes.to_string())
            .set("fetch.max.wait.ms", settings.consumer.fetch_max_wait.as_millis().to_string())
            .set("isolation.level", format!("{:?}", settings.consumer.isolation_level).to_lowercase())
            .create()?;

        Ok(Self { producer, consumer })
    }

    pub async fn publish(&self, topic: String, payload: serde_json::Value) -> Result<()> {
        let data = serde_json::to_vec(&payload)?;
        let record = rdkafka::producer::BaseRecord::to(&topic).payload(&data);
        self.producer.send(record).await.map_err(|(e, _)| e)?;
        Ok(())
    }

    pub async fn publish_with_key(&self, topic: String, key: Vec<u8>, payload: serde_json::Value) -> Result<()> {
        let data = serde_json::to_vec(&payload)?;
        let record = rdkafka::producer::BaseRecord::to(&topic).key(&key).payload(&data);
        self.producer.send(record).await.map_err(|(e, _)| e)?;
        Ok(())
    }

    pub async fn subscribe(&self, topics: &[&str]) -> Result<()> {
        self.consumer.subscribe(topics)?;
        Ok(())
    }

    pub async fn consume(&self) -> Result<rdkafka::Message<Vec<u8>>> {
        let msg = self.consumer.recv().await?;
        Ok(msg)
    }
}

/// Message serializer for different formats
pub struct MessageSerializer;

impl MessageSerializer {
    pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(value)?)
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T> {
        Ok(serde_json::from_slice(data)?)
    }

    pub fn serialize_to_string<T: Serialize>(value: &T) -> Result<String> {
        Ok(serde_json::to_string(value)?)
    }

    pub fn deserialize_from_string<T: for<'de> Deserialize<'de>>(data: &str) -> Result<T> {
        Ok(serde_json::from_str(data)?)
    }
}

/// Predefined event types for the platform
pub mod types {
    use super::*;
    use crate::domain::*;

    // User events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserCreated {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub email: String,
        pub name: String,
        pub locale: Locale,
        pub timezone: String,
    }
    impl_event!(UserCreated, "com.apexos.user.created", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserUpdated {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub changes: HashMap<String, serde_json::Value>,
    }
    impl_event!(UserUpdated, "com.apexos.user.updated", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserDeleted {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub deleted_by: UserId,
    }
    impl_event!(UserDeleted, "com.apexos.user.deleted", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserLogin {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub ip: IpAddr,
        pub user_agent: String,
        pub mfa_used: bool,
    }
    impl_event!(UserLogin, "com.apexos.user.login", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserLogout {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub session_id: SessionId,
    }
    impl_event!(UserLogout, "com.apexos.user.logout", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PasswordChanged {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub changed_by: UserId,
    }
    impl_event!(PasswordChanged, "com.apexos.user.password_changed", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MfaEnabled {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub method: String,
    }
    impl_event!(MfaEnabled, "com.apexos.user.mfa_enabled", "auth-service");

    // Organization events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OrganizationCreated {
        pub metadata: EventMetadata,
        pub organization_id: OrganizationId,
        pub name: String,
        pub domain: Option<String>,
        pub owner_id: UserId,
    }
    impl_event!(OrganizationCreated, "com.apexos.organization.created", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OrganizationUpdated {
        pub metadata: EventMetadata,
        pub organization_id: OrganizationId,
        pub changes: HashMap<String, serde_json::Value>,
    }
    impl_event!(OrganizationUpdated, "com.apexos.organization.updated", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OrganizationDeleted {
        pub metadata: EventMetadata,
        pub organization_id: OrganizationId,
        pub deleted_by: UserId,
    }
    impl_event!(OrganizationDeleted, "com.apexos.organization.deleted", "auth-service");

    // Membership events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MemberAdded {
        pub metadata: EventMetadata,
        pub organization_id: OrganizationId,
        pub user_id: UserId,
        pub role_ids: Vec<RoleId>,
        pub added_by: UserId,
    }
    impl_event!(MemberAdded, "com.apexos.member.added", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MemberRemoved {
        pub metadata: EventMetadata,
        pub organization_id: OrganizationId,
        pub user_id: UserId,
        pub removed_by: UserId,
    }
    impl_event!(MemberRemoved, "com.apexos.member.removed", "auth-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MemberRoleChanged {
        pub metadata: EventMetadata,
        pub organization_id: OrganizationId,
        pub user_id: UserId,
        pub old_role_ids: Vec<RoleId>,
        pub new_role_ids: Vec<RoleId>,
        pub changed_by: UserId,
    }
    impl_event!(MemberRoleChanged, "com.apexos.member.role_changed", "auth-service");

    // Course events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CourseCreated {
        pub metadata: EventMetadata,
        pub course_id: CourseId,
        pub organization_id: OrganizationId,
        pub title: String,
        pub created_by: UserId,
    }
    impl_event!(CourseCreated, "com.apexos.course.created", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CoursePublished {
        pub metadata: EventMetadata,
        pub course_id: CourseId,
        pub published_by: UserId,
    }
    impl_event!(CoursePublished, "com.apexos.course.published", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CourseUnpublished {
        pub metadata: EventMetadata,
        pub course_id: CourseId,
        pub unpublished_by: UserId,
    }
    impl_event!(CourseUnpublished, "com.apexos.course.unpublished", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CourseDeleted {
        pub metadata: EventMetadata,
        pub course_id: CourseId,
        pub deleted_by: UserId,
    }
    impl_event!(CourseDeleted, "com.apexos.course.deleted", "web-app");

    // Enrollment events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EnrollmentCreated {
        pub metadata: EventMetadata,
        pub enrollment_id: EnrollmentId,
        pub course_id: CourseId,
        pub user_id: UserId,
        pub enrolled_by: UserId,
        pub role: String,
    }
    impl_event!(EnrollmentCreated, "com.apexos.enrollment.created", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EnrollmentCompleted {
        pub metadata: EventMetadata,
        pub enrollment_id: EnrollmentId,
        pub completed_at: Timestamp,
    }
    impl_event!(EnrollmentCompleted, "com.apexos.enrollment.completed", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EnrollmentDropped {
        pub metadata: EventMetadata,
        pub enrollment_id: EnrollmentId,
        pub dropped_by: UserId,
        pub reason: Option<String>,
    }
    impl_event!(EnrollmentDropped, "com.apexos.enrollment.dropped", "web-app");

    // Assignment events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AssignmentCreated {
        pub metadata: EventMetadata,
        pub assignment_id: AssignmentId,
        pub course_id: CourseId,
        pub title: String,
        pub due_at: Option<Timestamp>,
        pub created_by: UserId,
    }
    impl_event!(AssignmentCreated, "com.apexos.assignment.created", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AssignmentUpdated {
        pub metadata: EventMetadata,
        pub assignment_id: AssignmentId,
        pub changes: HashMap<String, serde_json::Value>,
    }
    impl_event!(AssignmentUpdated, "com.apexos.assignment.updated", "web-app");

    // Submission events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubmissionCreated {
        pub metadata: EventMetadata,
        pub submission_id: SubmissionId,
        pub assignment_id: AssignmentId,
        pub user_id: UserId,
        pub content: serde_json::Value,
    }
    impl_event!(SubmissionCreated, "com.apexos.submission.created", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubmissionGraded {
        pub metadata: EventMetadata,
        pub submission_id: SubmissionId,
        pub graded_by: UserId,
        pub score: f64,
        pub feedback: Option<String>,
    }
    impl_event!(SubmissionGraded, "com.apexos.submission.graded", "grading-service");

    // Media events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MediaUploaded {
        pub metadata: EventMetadata,
        pub media_id: MediaId,
        pub filename: String,
        pub mime_type: String,
        pub size: u64,
        pub uploaded_by: UserId,
    }
    impl_event!(MediaUploaded, "com.apexos.media.uploaded", "media-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MediaProcessed {
        pub metadata: EventMetadata,
        pub media_id: MediaId,
        pub variants: Vec<MediaVariant>,
        pub processing_time_ms: u64,
    }
    impl_event!(MediaProcessed, "com.apexos.media.processed", "media-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MediaDeleted {
        pub metadata: EventMetadata,
        pub media_id: MediaId,
        pub deleted_by: UserId,
    }
    impl_event!(MediaDeleted, "com.apexos.media.deleted", "media-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MediaVariant {
        pub name: String,
        pub url: String,
        pub mime_type: String,
        pub size: u64,
        pub width: Option<u32>,
        pub height: Option<u32>,
        pub duration: Option<f64>,
    }

    // AI/Analytics events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiInferenceRequested {
        pub metadata: EventMetadata,
        pub request_id: Ulid,
        pub model: String,
        pub input: serde_json::Value,
        pub user_id: Option<UserId>,
    }
    impl_event!(AiInferenceRequested, "com.apexos.ai.inference.requested", "ai-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiInferenceCompleted {
        pub metadata: EventMetadata,
        pub request_id: Ulid,
        pub model: String,
        pub output: serde_json::Value,
        pub tokens_used: u64,
        pub latency_ms: u64,
    }
    impl_event!(AiInferenceCompleted, "com.apexos.ai.inference.completed", "ai-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RecommendationGenerated {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub recommendations: Vec<RecommendationItem>,
        pub context: String,
    }
    impl_event!(RecommendationGenerated, "com.apexos.recommendation.generated", "recommendation-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RecommendationItem {
        pub item_type: String,
        pub item_id: AggregateId,
        pub score: f64,
        pub reason: String,
    }

    // LTI events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LtiLaunchRequested {
        pub metadata: EventMetadata,
        pub launch_id: LTILaunchId,
        pub tool_id: String,
        pub user_id: UserId,
        pub context_id: String,
        pub resource_link_id: Option<String>,
    }
    impl_event!(LtiLaunchRequested, "com.apexos.lti.launch.requested", "lti-provider");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LtiLaunchCompleted {
        pub metadata: EventMetadata,
        pub launch_id: LTILaunchId,
        pub success: bool,
        pub error: Option<String>,
    }
    impl_event!(LtiLaunchCompleted, "com.apexos.lti.launch.completed", "lti-provider");

    // SCORM events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScormSessionStarted {
        pub metadata: EventMetadata,
        pub session_id: Ulid,
        pub package_id: SCOMPackageId,
        pub user_id: UserId,
        pub registration_id: String,
    }
    impl_event!(ScormSessionStarted, "com.apexos.scorm.session.started", "scorm-engine");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScormSessionCompleted {
        pub metadata: EventMetadata,
        pub session_id: Ulid,
        pub score: Option<f64>,
        pub success_status: Option<String>,
        pub completion_status: Option<String>,
        pub total_time: Duration,
    }
    impl_event!(ScormSessionCompleted, "com.apexos.scorm.session.completed", "scorm-engine");

    // xAPI events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct XapiStatementReceived {
        pub metadata: EventMetadata,
        pub statement_id: XAPIStatementId,
        pub statement: serde_json::Value,
    }
    impl_event!(XapiStatementReceived, "com.apexos.xapi.statement.received", "analytics-service");

    // Certificate events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CertificateIssued {
        pub metadata: EventMetadata,
        pub certificate_id: CertificateId,
        pub user_id: UserId,
        pub course_id: CourseId,
        pub issued_at: Timestamp,
        pub expires_at: Option<Timestamp>,
        pub credential_url: String,
    }
    impl_event!(CertificateIssued, "com.apexos.certificate.issued", "web-app");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CertificateRevoked {
        pub metadata: EventMetadata,
        pub certificate_id: CertificateId,
        pub revoked_by: UserId,
        pub reason: String,
    }
    impl_event!(CertificateRevoked, "com.apexos.certificate.revoked", "web-app");

    // Badge events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BadgeAwarded {
        pub metadata: EventMetadata,
        pub badge_id: BadgeId,
        pub user_id: UserId,
        pub evidence_url: Option<String>,
        pub awarded_by: UserId,
    }
    impl_event!(BadgeAwarded, "com.apexos.badge.awarded", "web-app");

    // Analytics events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PageViewed {
        pub metadata: EventMetadata,
        pub user_id: Option<UserId>,
        pub session_id: SessionId,
        pub path: String,
        pub referrer: Option<String>,
        pub duration_ms: Option<u64>,
    }
    impl_event!(PageViewed, "com.apexos.analytics.page_viewed", "analytics-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FeatureUsed {
        pub metadata: EventMetadata,
        pub user_id: UserId,
        pub feature: String,
        pub properties: HashMap<String, serde_json::Value>,
    }
    impl_event!(FeatureUsed, "com.apexos.analytics.feature_used", "analytics-service");

    // Audit events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuditLogCreated {
        pub metadata: EventMetadata,
        pub audit_id: AuditLogId,
        pub action: String,
        pub resource_type: String,
        pub resource_id: AggregateId,
        pub old_values: Option<serde_json::Value>,
        pub new_values: Option<serde_json::Value>,
    }
    impl_event!(AuditLogCreated, "com.apexos.audit.log_created", "auth-service");

    // Search events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchIndexUpdated {
        pub metadata: EventMetadata,
        pub index: String,
        pub document_id: String,
        pub operation: SearchOperation,
    }
    impl_event!(SearchIndexUpdated, "com.apexos.search.index_updated", "search-service");

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SearchOperation {
        Create,
        Update,
        Delete,
    }

    // Scheduler events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JobScheduled {
        pub metadata: EventMetadata,
        pub job_id: JobId,
        pub workflow_id: Option<WorkflowId>,
        pub scheduled_at: Timestamp,
        pub payload: serde_json::Value,
    }
    impl_event!(JobScheduled, "com.apexos.scheduler.job_scheduled", "scheduler-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JobCompleted {
        pub metadata: EventMetadata,
        pub job_id: JobId,
        pub result: serde_json::Value,
        pub duration_ms: u64,
    }
    impl_event!(JobCompleted, "com.apexos.scheduler.job_completed", "scheduler-service");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JobFailed {
        pub metadata: EventMetadata,
        pub job_id: JobId,
        pub error: String,
        pub retry_count: u32,
        pub will_retry: bool,
    }
    impl_event!(JobFailed, "com.apexos.scheduler.job_failed", "scheduler-service");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    #[test]
    fn event_metadata_creation() {
        let metadata = EventMetadata::new("test-service", "com.test.event")
            .with_correlation_id(CorrelationId::new())
            .with_tenant(TenantId::new());

        assert_eq!(metadata.source, "test-service");
        assert_eq!(metadata.event_type, "com.test.event");
        assert!(metadata.correlation_id.is_some());
        assert!(metadata.tenant_id.is_some());
    }

    #[test]
    fn nats_subject_generation() {
        let metadata = EventMetadata::new("auth.service", "com.apexos.user.created");
        assert_eq!(metadata.nats_subject(), "events.auth_service.com_apexos_user_created");
    }

    #[test]
    fn event_envelope() {
        let event = types::UserCreated {
            metadata: EventMetadata::new("auth-service", "com.apexos.user.created"),
            user_id: UserId::new(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            locale: Locale::default(),
            timezone: "UTC".to_string(),
        };

        let envelope = event.into_envelope();
        assert_eq!(envelope.metadata.event_type, "com.apexos.user.created");
    }
}