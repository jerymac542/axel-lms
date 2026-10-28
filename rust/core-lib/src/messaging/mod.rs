//! Messaging
//!
//! NATS JetStream and Apache Kafka clients with serialization and patterns.

use crate::config::{NatsConfig, KafkaConfig, StreamConfig, ConsumerConfig, AckPolicy, ReplayPolicy, RetentionPolicy, StorageType, ProducerConfig, Acks, CompressionType, KafkaConsumerConfig, AutoOffsetReset, IsolationLevel};
use crate::domain::{TenantId, UserId};
use crate::errors::{ErrorCode, Result};
use crate::events::{Event, EventEnvelope, EventMetadata, MessageBus, NatsClient, KafkaClient, MessageSerializer};
use crate::observability::{tracer, CommonMetrics};
use async_nats::{Client, ConnectOptions, HeaderMap, Message, jetstream};
use futures::{StreamExt, SinkExt};
use opentelemetry::trace::Tracer;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    consumer::{StreamConsumer, CommitMode},
    Message as KafkaMessage, ToBytes,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info, instrument, warn};

/// Extended NATS client with JetStream support
#[derive(Clone)]
pub struct ExtendedNatsClient {
    pub client: Client,
    pub jetstream: Option<jetstream::Context>,
    pub metrics: Option<CommonMetrics>,
}

impl ExtendedNatsClient {
    pub async fn connect(config: &NatsConfig) -> Result<Self> {
        let mut options = ConnectOptions::new()
            .max_reconnects(config.max_reconnects)
            .reconnect_delay(config.reconnect_delay)
            .ping_interval(config.ping_interval);

        if let (Some(user), Some(pass)) = (&config.username, &config.password) {
            options = options.credentials(user, pass.expose_secret());
        }
        if let Some(token) = &config.token {
            options = options.token(token.expose_secret());
        }
        if let Some(nkey) = &config.nkey {
            options = options.nkey(nkey.expose_secret());
        }
        if config.tls {
            options = options.tls(async_nats::tls::default_client_config()?);
        }

        let client = options.connect(config.urls.first().unwrap()).await?;

        let jetstream = if config.jetstream.enabled {
            Some(jetstream::new(client.clone()))
        } else {
            None
        };

        Ok(Self { client, jetstream, metrics: None })
    }

    pub fn with_metrics(mut self, metrics: CommonMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    #[instrument(skip(self, payload))]
    pub async fn publish<T: Serialize>(&self, subject: &str, payload: &T) -> Result<()> {
        let data = serde_json::to_vec(payload)?;
        
        if let Some(metrics) = &self.metrics {
            metrics.message_published.add(1, &[KeyValue::new("subject", subject.to_string())]);
        }

        self.client.publish(subject.to_string(), data.into()).await?;
        Ok(())
    }

    #[instrument(skip(self, payload))]
    pub async fn publish_with_headers<T: Serialize>(&self, subject: &str, headers: HashMap<String, String>, payload: &T) -> Result<()> {
        let data = serde_json::to_vec(payload)?;
        let mut message = Message::builder().payload(data.into());
        
        for (k, v) in headers {
            message = message.header(k, v);
        }
        
        if let Some(metrics) = &self.metrics {
            metrics.message_published.add(1, &[KeyValue::new("subject", subject.to_string())]);
        }

        self.client.publish(subject.to_string(), message.build()).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn request<T: Serialize, R: for<'de> Deserialize<'de>>(&self, subject: &str, payload: &T, timeout: Duration) -> Result<R> {
        let data = serde_json::to_vec(payload)?;
        let response = self.client.request(subject.to_string(), data.into()).await?;
        let result = serde_json::from_slice(&response.payload)?;
        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn subscribe(&self, subject: &str, queue_group: Option<&str>) -> Result<async_nats::Subscriber> {
        let subscriber = if let Some(qg) = queue_group {
            self.client.queue_subscribe(subject.to_string(), qg.to_string()).await?
        } else {
            self.client.subscribe(subject.to_string()).await?
        };
        Ok(subscriber)
    }

    #[instrument(skip(self))]
    pub async fn subscribe_with_handler<F, Fut>(&self, subject: &str, queue_group: Option<&str>, handler: F) -> Result<async_nats::Subscriber>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let mut subscriber = self.subscribe(subject, queue_group).await?;
        
        tokio::spawn(async move {
            while let Some(message) = subscriber.next().await {
                if let Err(e) = handler(message).await {
                    error!("Message handler error: {}", e);
                }
            }
        });

        Ok(subscriber)
    }

    pub async fn create_stream(&self, config: &StreamConfig) -> Result<jetstream::Stream> {
        let js = self.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "JetStream not enabled"))?;
        
        let stream_config = jetstream::stream::Config {
            name: config.name.clone(),
            subjects: config.subjects.clone(),
            retention: match config.retention {
                RetentionPolicy::Limits => jetstream::stream::RetentionPolicy::Limits,
                RetentionPolicy::Interest => jetstream::stream::RetentionPolicy::Interest,
                RetentionPolicy::WorkQueue => jetstream::stream::RetentionPolicy::WorkQueue,
            },
            max_consumers: config.max_consumers,
            max_msgs: config.max_msgs,
            max_bytes: config.max_bytes,
            max_age: config.max_age,
            max_msg_size: config.max_msg_size,
            storage: match config.storage {
                StorageType::File => jetstream::stream::StorageType::File,
                StorageType::Memory => jetstream::stream::StorageType::Memory,
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

    pub async fn create_consumer(&self, stream: &str, config: &ConsumerConfig) -> Result<jetstream::Consumer> {
        let js = self.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "JetStream not enabled"))?;
        
        let consumer_config = jetstream::consumer::Config {
            durable_name: config.durable_name.clone(),
            ack_policy: match config.ack_policy {
                AckPolicy::None => jetstream::consumer::AckPolicy::None,
                AckPolicy::All => jetstream::consumer::AckPolicy::All,
                AckPolicy::Explicit => jetstream::consumer::AckPolicy::Explicit,
            },
            ack_wait: config.ack_wait,
            max_deliver: config.max_deliver,
            filter_subject: config.filter_subject.clone(),
            replay_policy: match config.replay_policy {
                ReplayPolicy::Instant => jetstream::consumer::ReplayPolicy::Instant,
                ReplayPolicy::Original => jetstream::consumer::ReplayPolicy::Original,
                ReplayPolicy::New => jetstream::consumer::ReplayPolicy::New,
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

    pub async fn consume_stream(&self, stream: &str, consumer: &str) -> Result<jetstream::Consumer> {
        let js = self.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "JetStream not enabled"))?;
        Ok(js.get_consumer(stream, consumer).await?)
    }

    pub async fn publish_event<T: Event>(&self, event: T) -> Result<()> {
        let envelope = event.into_envelope();
        let subject = envelope.metadata.nats_subject();
        self.publish(&subject, &envelope).await
    }

    pub async fn subscribe_events<F, Fut>(&self, subject_prefix: &str, queue_group: &str, handler: F) -> Result<()>
    where
        F: Fn(EventEnvelope<serde_json::Value>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let subject = format!("{}.>", subject_prefix);
        let mut subscriber = self.subscribe(&subject, Some(queue_group)).await?;

        while let Some(message) = subscriber.next().await {
            if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&message.payload) {
                if let Err(e) = handler(envelope).await {
                    error!("Event handler error: {}", e);
                    if let Some(metrics) = &self.metrics {
                        metrics.message_errors.add(1, &[KeyValue::new("subject", subject.clone())]);
                    }
                } else if let Some(metrics) = &self.metrics {
                    metrics.message_consumed.add(1, &[KeyValue::new("subject", subject.clone())]);
                }
            }
        }

        Ok(())
    }
}

/// Extended Kafka client
#[derive(Clone)]
pub struct ExtendedKafkaClient {
    pub producer: FutureProducer,
    pub consumer: StreamConsumer,
    pub metrics: Option<CommonMetrics>,
}

impl ExtendedKafkaClient {
    pub async fn connect(config: &KafkaConfig) -> Result<Self> {
        let producer: FutureProducer = rdkafka::ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("client.id", &config.client_id)
            .set("message.timeout.ms", config.producer.delivery_timeout.as_millis().to_string())
            .set("request.timeout.ms", config.producer.request_timeout.as_millis().to_string())
            .set("enable.idempotence", config.producer.enable_idempotence.to_string())
            .set("max.in.flight.requests.per.connection", config.producer.max_in_flight.to_string())
            .set("compression.type", format!("{:?}", config.producer.compression).to_lowercase())
            .set("linger.ms", config.producer.linger_ms.to_string())
            .set("batch.size", config.producer.batch_size.to_string())
            .set("buffer.memory", config.producer.buffer_memory.to_string())
            .set("retries", config.producer.retries.to_string())
            .set("acks", format!("{:?}", config.producer.acks).to_lowercase())
            .create()?;

        let consumer: StreamConsumer = rdkafka::ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("client.id", &config.client_id)
            .set("group.id", &config.group_id)
            .set("enable.auto.commit", config.consumer.enable_auto_commit.to_string())
            .set("auto.offset.reset", format!("{:?}", config.consumer.auto_offset_reset).to_lowercase())
            .set("session.timeout.ms", config.consumer.session_timeout.as_millis().to_string())
            .set("heartbeat.interval.ms", config.consumer.heartbeat_interval.as_millis().to_string())
            .set("max.poll.records", config.consumer.max_poll_records.to_string())
            .set("max.poll.interval.ms", config.consumer.max_poll_interval.as_millis().to_string())
            .set("fetch.min.bytes", config.consumer.fetch_min_bytes.to_string())
            .set("fetch.max.wait.ms", config.consumer.fetch_max_wait.as_millis().to_string())
            .set("isolation.level", format!("{:?}", config.consumer.isolation_level).to_lowercase())
            .create()?;

        Ok(Self { producer, consumer, metrics: None })
    }

    pub fn with_metrics(mut self, metrics: CommonMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    #[instrument(skip(self, payload))]
    pub async fn publish<T: Serialize>(&self, topic: &str, payload: &T) -> Result<()> {
        let data = serde_json::to_vec(payload)?;
        let record = FutureRecord::to(topic).payload(&data);
        
        if let Some(metrics) = &self.metrics {
            metrics.message_published.add(1, &[KeyValue::new("topic", topic.to_string())]);
        }

        self.producer.send(record).await.map_err(|(e, _)| e)?;
        Ok(())
    }

    #[instrument(skip(self, payload))]
    pub async fn publish_with_key<T: Serialize>(&self, topic: &str, key: &[u8], payload: &T) -> Result<()> {
        let data = serde_json::to_vec(payload)?;
        let record = FutureRecord::to(topic).key(key).payload(&data);
        
        if let Some(metrics) = &self.metrics {
            metrics.message_published.add(1, &[KeyValue::new("topic", topic.to_string())]);
        }

        self.producer.send(record).await.map_err(|(e, _)| e)?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn subscribe(&self, topics: &[&str]) -> Result<()> {
        self.consumer.subscribe(topics)?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn consume(&self) -> Result<rdkafka::Message<Vec<u8>>> {
        let msg = self.consumer.recv().await?;
        
        if let Some(metrics) = &self.metrics {
            metrics.message_consumed.add(1, &[KeyValue::new("topic", msg.topic().to_string())]);
        }

        Ok(msg)
    }

    pub async fn consume_batch(&self, max_batch: usize, timeout: Duration) -> Result<Vec<rdkafka::Message<Vec<u8>>>> {
        let mut messages = Vec::new();
        let start = std::time::Instant::now();

        while messages.len() < max_batch && start.elapsed() < timeout {
            match self.consumer.recv().await {
                Ok(msg) => messages.push(msg),
                Err(e) => {
                    error!("Kafka consume error: {}", e);
                    break;
                }
            }
        }

        if let Some(metrics) = &self.metrics {
            metrics.message_consumed.add(messages.len() as u64, &[]);
        }

        Ok(messages)
    }

    pub async fn commit(&self, mode: CommitMode) -> Result<()> {
        self.consumer.commit_consumer_state(mode)?;
        Ok(())
    }

    pub async fn publish_event<T: Event>(&self, event: T) -> Result<()> {
        let envelope = event.into_envelope();
        let topic = envelope.metadata.kafka_topic();
        let key = envelope.metadata.event_id.to_string().into_bytes();
        self.publish_with_key(&topic, &key, &envelope).await
    }

    pub async fn consume_events<F, Fut>(&self, topics: &[&str], handler: F) -> Result<()>
    where
        F: Fn(EventEnvelope<serde_json::Value>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        self.subscribe(topics).await?;

        loop {
            match self.consume().await {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(payload) {
                            if let Err(e) = handler(envelope).await {
                                error!("Event handler error: {}", e);
                                if let Some(metrics) = &self.metrics {
                                    metrics.message_errors.add(1, &[KeyValue::new("topic", msg.topic().to_string())]);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Kafka consume error: {}", e);
                    if let Some(metrics) = &self.metrics {
                        metrics.message_errors.add(1, &[]);
                    }
                }
            }
        }
    }
}

/// Message patterns and utilities
pub mod patterns {
    use super::*;
    use async_nats::Message;
    use futures::FutureExt;
    use std::future::Future;
    use tokio::sync::oneshot;

    /// Request-reply pattern over NATS
    pub struct RequestReply {
        client: ExtendedNatsClient,
        timeout: Duration,
    }

    impl RequestReply {
        pub fn new(client: ExtendedNatsClient, timeout: Duration) -> Self {
            Self { client, timeout }
        }

        pub async fn request<T: Serialize, R: for<'de> Deserialize<'de>>(&self, subject: &str, payload: &T) -> Result<R> {
            self.client.request(subject, payload, self.timeout).await
        }
    }

    /// Publish-subscribe with automatic acknowledgment
    pub struct PubSub {
        client: ExtendedNatsClient,
        subject: String,
        queue_group: String,
    }

    impl PubSub {
        pub fn new(client: ExtendedNatsClient, subject: String, queue_group: String) -> Self {
            Self { client, subject, queue_group }
        }

        pub async fn publish<T: Serialize>(&self, payload: &T) -> Result<()> {
            self.client.publish(&self.subject, payload).await
        }

        pub async fn subscribe<F, Fut>(&self, handler: F) -> Result<()>
        where
            F: Fn(Message) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send + 'static,
        {
            let mut subscriber = self.client.subscribe(&self.subject, Some(&self.queue_group)).await?;
            
            while let Some(message) = subscriber.next().await {
                if let Err(e) = handler(message).await {
                    error!("PubSub handler error: {}", e);
                }
            }
            
            Ok(())
        }
    }

    /// Work queue pattern with JetStream
    pub struct WorkQueue {
        client: ExtendedNatsClient,
        stream: String,
        consumer: String,
    }

    impl WorkQueue {
        pub async fn new(client: ExtendedNatsClient, stream: String, consumer: String) -> Result<Self> {
            // Ensure stream exists
            let stream_config = StreamConfig {
                name: stream.clone(),
                subjects: vec![format!("{}.>", stream)],
                retention: RetentionPolicy::WorkQueue,
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
            };
            client.create_stream(&stream_config).await?;

            // Ensure consumer exists
            let consumer_config = ConsumerConfig {
                durable_name: Some(consumer.clone()),
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
            };
            client.create_consumer(&stream, &consumer_config).await?;

            Ok(Self { client, stream, consumer })
        }

        pub async fn enqueue<T: Serialize>(&self, payload: &T) -> Result<()> {
            let subject = format!("{}.job", self.stream);
            self.client.publish(&subject, payload).await
        }

        pub async fn process<F, Fut, T>(&self, handler: F) -> Result<()>
        where
            F: Fn(T) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send + 'static,
            T: for<'de> Deserialize<'de> + Send + 'static,
        {
            let js = self.client.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "JetStream not enabled"))?;
            let consumer = js.get_consumer(&self.stream, &self.consumer).await?;
            
            let mut messages = consumer.messages().await?;
            
            while let Some(message) = messages.next().await {
                let message = message?;
                let payload: T = serde_json::from_slice(&message.payload)?;
                
                match handler(payload).await {
                    Ok(_) => {
                        message.ack().await?;
                    }
                    Err(e) => {
                        error!("WorkQueue handler error: {}", e);
                        message.nack(None).await?;
                    }
                }
            }
            
            Ok(())
        }
    }

    /// Event sourcing pattern
    pub struct EventStore {
        client: ExtendedNatsClient,
        stream: String,
    }

    impl EventStore {
        pub async fn new(client: ExtendedNatsClient, stream: String) -> Result<Self> {
            let stream_config = StreamConfig {
                name: stream.clone(),
                subjects: vec![format!("{}.>", stream)],
                retention: RetentionPolicy::Limits,
                max_consumers: -1,
                max_msgs: -1,
                max_bytes: -1,
                max_age: Duration::from_secs(86400 * 365),
                max_msg_size: 1024 * 1024,
                storage: StorageType::File,
                replicas: 3,
                no_ack: false,
                duplicate_window: Duration::from_secs(120),
                placement: None,
                mirror: None,
                sources: vec![],
                allow_rollup: false,
                deny_delete: false,
                deny_purge: false,
            };
            client.create_stream(&stream_config).await?;

            Ok(Self { client, stream })
        }

        pub async fn append<T: Event>(&self, event: T) -> Result<u64> {
            let envelope = event.into_envelope();
            let subject = envelope.metadata.nats_subject();
            let data = serde_json::to_vec(&envelope)?;
            
            let js = self.client.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "JetStream not enabled"))?;
            let ack = js.publish(subject, data.into()).await?;
            Ok(ack.sequence)
        }

        pub async fn read_from(&self, sequence: u64) -> Result<Vec<EventEnvelope<serde_json::Value>>> {
            let js = self.client.jetstream.as_ref().ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "JetStream not enabled"))?;
            let consumer = js.get_consumer(&self.stream, "event-store-reader").await?;
            
            let config = jetstream::consumer::Config {
                durable_name: Some("event-store-reader".to_string()),
                ack_policy: jetstream::consumer::AckPolicy::None,
                filter_subject: None,
                replay_policy: jetstream::consumer::ReplayPolicy::Original,
                ..Default::default()
            };
            
            // This is a simplified implementation
            // In practice, you'd use get_messages with a batch
            Ok(vec![])
        }
    }
}

/// Dead letter queue handling
pub mod dead_letter {
    use super::*;
    use serde_json::Value;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeadLetter {
        pub original_subject: String,
        pub original_payload: Value,
        pub error: String,
        pub attempts: u32,
        pub failed_at: crate::domain::Timestamp,
        pub headers: HashMap<String, String>,
    }

    pub async fn handle_dead_letter(
        nats: &ExtendedNatsClient,
        dlq_subject: &str,
        dead_letter: DeadLetter,
    ) -> Result<()> {
        nats.publish(dlq_subject, &dead_letter).await
    }

    pub async fn retry_from_dlq(
        nats: &ExtendedNatsClient,
        dlq_subject: &str,
        original_subject: &str,
    ) -> Result<()> {
        let mut subscriber = nats.subscribe(dlq_subject, Some("dlq-retry")).await?;
        
        while let Some(message) = subscriber.next().await {
            if let Ok(dl) = serde_json::from_slice::<DeadLetter>(&message.payload) {
                if dl.original_subject == original_subject {
                    nats.publish(&dl.original_subject, &dl.original_payload).await?;
                    message.ack().await?;
                }
            }
        }
        
        Ok(())
    }
}

/// Message tracing integration
pub mod tracing {
    use super::*;
    use opentelemetry::{Context, trace::SpanKind};
    use opentelemetry::trace::Tracer;

    pub fn inject_trace_context<T: Serialize>(tracer: &Tracer, subject: &str, payload: &T) -> Result<(Vec<u8>, HashMap<String, String>)> {
        let data = serde_json::to_vec(payload)?;
        let mut headers = HashMap::new();
        
        let cx = Context::current();
        crate::observability::propagation::inject_context(&cx, &mut headers);
        
        // Add messaging-specific attributes
        headers.insert("messaging.system".to_string(), "nats".to_string());
        headers.insert("messaging.destination".to_string(), subject.to_string());
        headers.insert("messaging.operation".to_string(), "publish".to_string());
        
        Ok((data, headers))
    }

    pub fn extract_trace_context(headers: &HashMap<String, String>) -> Context {
        crate::observability::propagation::extract_context(headers)
    }

    pub fn create_consumer_span(tracer: &Tracer, subject: &str) -> opentelemetry::trace::Span {
        tracer.start_with_context(
            format!("consume {}", subject),
            vec![],
            &Context::current(),
            SpanKind::Consumer,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    #[test]
    fn nats_config_defaults() {
        let config = NatsConfig {
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
        };

        assert_eq!(config.urls.len(), 1);
        assert!(config.jetstream.enabled);
    }
}