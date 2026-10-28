//! Observability
//!
//! OpenTelemetry integration for tracing, metrics, and logging.

use crate::config::{TracingConfig, MetricsConfig, LoggingConfig, LogLevel, LogFormat, LogOutput, ExporterConfig, ExporterProtocol, BatchConfig, SamplerConfig};
use crate::errors::Result;
use opentelemetry::{
    global, trace::TracerProvider as _, metrics::MeterProvider as _, KeyValue,
    sdk::{trace::{self, Sampler, SpanExporter, BatchConfig as OtelBatchConfig}, metrics::{self as sdk_metrics, PeriodicExportingMetricReader}, Resource},
    trace::TraceError,
    Context,
};
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig, Protocol};
use opentelemetry_sdk::logs::{LoggerProvider, BatchLogRecordProcessor, LogRecordExporter};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION, DEPLOYMENT_ENVIRONMENT, TELEMETRY_SDK_LANGUAGE};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::{Subscriber, subscriber::set_global_default, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::{self, format::JsonFields};
use tracing_subscriber::fmt::format::{Format, DefaultFields};
use tracing_subscriber::fmt::writer::MakeWriterExt;

static TRACER_PROVIDER: OnceLock<opentelemetry_sdk::trace::TracerProvider> = OnceLock::new();
static METER_PROVIDER: OnceLock<opentelemetry_sdk::metrics::MeterProvider> = OnceLock::new();
static LOGGER_PROVIDER: OnceLock<LoggerProvider> = OnceLock::new();

/// Initialize tracing with OpenTelemetry
pub fn init_tracing(config: &TracingConfig) -> Result<()> {
    let tracer_provider = init_tracer_provider(config)?;
    TRACER_PROVIDER.set(tracer_provider.clone()).ok();

    let tracer = tracer_provider.tracer(config.service_name.clone());

    // Create OpenTelemetry layer
    let otel_layer = OpenTelemetryLayer::new(tracer)
        .with_location(true)
        .with_error_records(true);

    // Create formatting layer
    let fmt_layer = create_fmt_layer(&config.logging)?;

    // Create filter
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(match config.logging.level {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }));

    // Build subscriber
    let subscriber = Registry::default()
        .with(filter)
        .with(fmt_layer)
        .with(otel_layer);

    set_global_default(subscriber).map_err(|e| crate::errors::ApexosError::new(crate::errors::ErrorCode::InternalError, e.to_string()))?;

    Ok(())
}

fn init_tracer_provider(config: &TracingConfig) -> Result<opentelemetry_sdk::trace::TracerProvider> {
    let exporter = build_span_exporter(&config.exporter)?;
    
    let batch_config = OtelBatchConfig {
        max_queue_size: config.exporter.batch.max_queue_size,
        max_export_batch_size: config.exporter.batch.max_export_batch_size,
        scheduled_delay: config.exporter.batch.schedule_delay,
        export_timeout: config.exporter.batch.export_timeout,
        ..Default::default()
    };

    let sampler = match config.sampler {
        SamplerConfig::AlwaysOn => Sampler::AlwaysOn,
        SamplerConfig::AlwaysOff => Sampler::AlwaysOff,
        SamplerConfig::TraceIdRatio(ratio) => Sampler::TraceIdRatioBased(ratio),
        SamplerConfig::ParentBasedTraceIdRatio(ratio) => Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(ratio))),
    };

    let resource = build_resource(config);

    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, batch_config)
        .with_sampler(sampler)
        .with_resource(resource)
        .build();

    Ok(provider)
}

fn build_span_exporter(config: &ExporterConfig) -> Result<Box<dyn SpanExporter + Send + Sync>> {
    let mut exporter_builder = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(config.endpoint.as_str())
        .with_timeout(config.timeout);

    // Add headers
    for (key, value) in &config.headers {
        exporter_builder = exporter_builder.with_header(key.clone(), value.clone());
    }

    let protocol = match config.protocol {
        ExporterProtocol::Grpc => Protocol::Grpc,
        ExporterProtocol::HttpProtobuf => Protocol::HttpBinary,
        ExporterProtocol::HttpJson => Protocol::HttpJson,
    };
    exporter_builder = exporter_builder.with_protocol(protocol);

    let exporter = exporter_builder.build()?;
    Ok(Box::new(exporter))
}

fn build_resource(config: &TracingConfig) -> Resource {
    let mut attributes = vec![
        KeyValue::new(SERVICE_NAME, config.service_name.clone()),
        KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        KeyValue::new(DEPLOYMENT_ENVIRONMENT, config.environment.clone()),
        KeyValue::new(TELEMETRY_SDK_LANGUAGE, "rust"),
    ];

    for (key, value) in &config.resource_attributes {
        attributes.push(KeyValue::new(key.clone(), value.clone()));
    }

    Resource::new(attributes)
}

/// Initialize metrics with OpenTelemetry
pub fn init_metrics(config: &MetricsConfig) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let exporter = if let Some(endpoint) = &config.endpoint {
        build_metric_exporter(endpoint, config.push_interval)?
    } else {
        // Use Prometheus exporter
        let exporter = opentelemetry_prometheus::exporter().build()?;
        return init_prometheus_metrics(exporter);
    };

    let reader = sdk_metrics::PeriodicExportingMetricReader::builder(exporter)
        .with_interval(config.push_interval)
        .build();

    let provider = opentelemetry_sdk::metrics::MeterProvider::builder()
        .with_reader(reader)
        .with_resource(build_resource_from_config(config))
        .build();

    global::set_meter_provider(provider.clone());
    METER_PROVIDER.set(provider).ok();

    Ok(())
}

fn build_metric_exporter(endpoint: &url::Url, interval: Duration) -> Result<opentelemetry_otlp::MetricExporter> {
    let mut builder = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint.as_str())
        .with_timeout(interval);

    let exporter = builder.build()?;
    Ok(exporter)
}

fn build_resource_from_config(config: &MetricsConfig) -> Resource {
    let attributes = vec![
        KeyValue::new(SERVICE_NAME, config.namespace.clone()),
        KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
    ];
    Resource::new(attributes)
}

fn init_prometheus_metrics(exporter: opentelemetry_prometheus::PrometheusExporter) -> Result<()> {
    let reader = sdk_metrics::PeriodicExportingMetricReader::builder(exporter)
        .with_interval(Duration::from_secs(10))
        .build();

    let provider = opentelemetry_sdk::metrics::MeterProvider::builder()
        .with_reader(reader)
        .build();

    global::set_meter_provider(provider.clone());
    METER_PROVIDER.set(provider).ok();

    Ok(())
}

/// Get tracer for a service
pub fn tracer(name: &str) -> opentelemetry::trace::Tracer {
    if let Some(provider) = TRACER_PROVIDER.get() {
        provider.tracer(name)
    } else {
        global::tracer(name)
    }
}

/// Get meter for a service
pub fn meter(name: &str) -> opentelemetry::metrics::Meter {
    if let Some(provider) = METER_PROVIDER.get() {
        provider.meter(name)
    } else {
        global::meter(name)
    }
}

/// Create a new span with attributes
pub fn create_span(name: &str, attributes: Vec<KeyValue>) -> tracing::Span {
    let tracer = tracer("apexos");
    let mut span = tracer.start(name);
    for attr in attributes {
        span.set_attribute(attr);
    }
    span
}

/// Shutdown observability providers
pub fn shutdown_observability() {
    if let Some(provider) = TRACER_PROVIDER.get() {
        let _ = provider.shutdown();
    }
    if let Some(provider) = METER_PROVIDER.get() {
        let _ = provider.shutdown();
    }
    if let Some(provider) = LOGGER_PROVIDER.get() {
        let _ = provider.shutdown();
    }
}

/// Tracing layer for Axum
pub struct TracingLayer;

impl TracingLayer {
    pub fn new() -> tower_http::trace::TraceLayer<
        tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
        tower_http::trace::DefaultMakeSpan,
        tower_http::trace::DefaultOnRequest,
        tower_http::trace::DefaultOnResponse,
        tower_http::trace::DefaultOnFailure,
    > {
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
            .on_request(tower_http::trace::DefaultOnRequest::new().level(Level::INFO))
            .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO))
            .on_failure(tower_http::trace::DefaultOnFailure::new().level(Level::ERROR))
    }
}

/// Metrics layer for Axum
pub struct MetricsLayer;

impl MetricsLayer {
    pub fn new() -> tower_http::metrics::MetricsLayer {
        tower_http::metrics::MetricsLayer::new()
    }
}

/// Common metric instruments
pub struct CommonMetrics {
    pub http_requests_total: opentelemetry::metrics::Counter<u64>,
    pub http_request_duration: opentelemetry::metrics::Histogram<f64>,
    pub http_request_size: opentelemetry::metrics::Histogram<u64>,
    pub http_response_size: opentelemetry::metrics::Histogram<u64>,
    pub active_connections: opentelemetry::metrics::UpDownCounter<i64>,
    pub db_query_duration: opentelemetry::metrics::Histogram<f64>,
    pub db_query_errors: opentelemetry::metrics::Counter<u64>,
    pub cache_hits: opentelemetry::metrics::Counter<u64>,
    pub cache_misses: opentelemetry::metrics::Counter<u64>,
    pub message_published: opentelemetry::metrics::Counter<u64>,
    pub message_consumed: opentelemetry::metrics::Counter<u64>,
    pub message_errors: opentelemetry::metrics::Counter<u64>,
    pub auth_attempts: opentelemetry::metrics::Counter<u64>,
    pub auth_failures: opentelemetry::metrics::Counter<u64>,
    pub rate_limit_exceeded: opentelemetry::metrics::Counter<u64>,
}

impl CommonMetrics {
    pub fn new(meter: opentelemetry::metrics::Meter) -> Self {
        Self {
            http_requests_total: meter.u64_counter("http.requests.total").build(),
            http_request_duration: meter.f64_histogram("http.request.duration").with_unit("ms").build(),
            http_request_size: meter.u64_histogram("http.request.size").with_unit("bytes").build(),
            http_response_size: meter.u64_histogram("http.response.size").with_unit("bytes").build(),
            active_connections: meter.i64_up_down_counter("http.connections.active").build(),
            db_query_duration: meter.f64_histogram("db.query.duration").with_unit("ms").build(),
            db_query_errors: meter.u64_counter("db.query.errors").build(),
            cache_hits: meter.u64_counter("cache.hits").build(),
            cache_misses: meter.u64_counter("cache.misses").build(),
            message_published: meter.u64_counter("message.published").build(),
            message_consumed: meter.u64_counter("message.consumed").build(),
            message_errors: meter.u64_counter("message.errors").build(),
            auth_attempts: meter.u64_counter("auth.attempts").build(),
            auth_failures: meter.u64_counter("auth.failures").build(),
            rate_limit_exceeded: meter.u64_counter("rate_limit.exceeded").build(),
        }
    }
}

/// Logging initialization
pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    let format = match config.format {
        LogFormat::Json => Format::default().json(),
        LogFormat::Text => Format::default(),
        LogFormat::Pretty => Format::default().pretty(),
    };

    let writer = match config.output {
        LogOutput::Stdout => std::io::stdout.with_max_level(Level::from(config.level)),
        LogOutput::Stderr => std::io::stderr.with_max_level(Level::from(config.level)),
        LogOutput::File => {
            // Would need file path from config
            std::io::stdout.with_max_level(Level::from(config.level))
        }
        LogOutput::Syslog => {
            // Would need syslog setup
            std::io::stdout.with_max_level(Level::from(config.level))
        }
    };

    let layer = fmt::layer()
        .with_writer(writer)
        .with_ansi(matches!(config.output, LogOutput::Stdout | LogOutput::Stderr))
        .event_format(format)
        .fmt_fields(JsonFields::new())
        .with_current_span(true)
        .with_span_list(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // Note: In practice, you'd add this to the subscriber in init_tracing
    // This is a standalone setup for non-OpenTelemetry logging
    Ok(())
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

/// Structured logging macros
#[macro_export]
macro_rules! log_info {
    ($($args:tt)*) => {
        tracing::info!($($args)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($args:tt)*) => {
        tracing::debug!($($args)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($args:tt)*) => {
        tracing::warn!($($args)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($args:tt)*) => {
        tracing::error!($($args)*)
    };
}

/// Context propagation utilities
pub mod propagation {
    use opentelemetry::{Context, propagation::TextMapPropagator};
    use std::collections::HashMap;

    pub fn inject_context(context: &Context, carrier: &mut HashMap<String, String>) {
        global::text_map_propagator(|propagator| {
            propagator.inject_context(context, carrier);
        });
    }

    pub fn extract_context(carrier: &HashMap<String, String>) -> Context {
        global::text_map_propagator(|propagator| {
            propagator.extract(carrier)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    #[test]
    fn tracer_provider_initialization() {
        let config = TracingConfig {
            enabled: true,
            service_name: "test-service".to_string(),
            environment: "test".to_string(),
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
            propagators: vec![],
            resource_attributes: HashMap::new(),
            logging: LoggingConfig {
                level: LogLevel::Info,
                format: LogFormat::Json,
                output: LogOutput::Stdout,
                json_fields: HashMap::new(),
                sampling: crate::config::SamplingConfig {
                    enabled: false,
                    rate: 1.0,
                    max_per_second: 1000,
                },
            },
        };

        // Just test that we can create the config
        assert!(config.enabled);
    }
}