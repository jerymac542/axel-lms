//! Apexos Core Library
//!
//! Shared types, events, validation, crypto, and utilities for Axel LMS services.
//!
//! # Architecture
//!
//! This crate provides the foundational building blocks used across all Apexos services:
//! - **Domain types**: Core identifiers, value objects, aggregates
//! - **Events**: CloudEvents-compatible event definitions for inter-service communication
//! - **Validation**: Garde-based validation with custom validators
//! - **Crypto**: Encryption, signing, key management, JWT/OAuth2 utilities
//! - **Observability**: OpenTelemetry integration, structured logging, metrics
//! - **Messaging**: NATS JetStream, Kafka producers/consumers with serialization
//! - **HTTP**: Axum middleware, error handling, authentication extractors
//! - **Storage**: S3/MinIO, database connection pooling, Redis utilities
//! - **Tenancy**: Multi-tenant context, organization isolation
//! - **RBAC**: Role-based access control, permissions, policies

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::too_many_lines,
    clippy::type_complexity
)]

pub mod config;
pub mod domain;
pub mod events;
pub mod validation;
pub mod crypto;
pub mod observability;
pub mod messaging;
pub mod http;
pub mod storage;
pub mod errors;
pub mod utils;
pub mod tenancy;
pub mod rbac;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::config::Settings;
    pub use crate::domain::{
        AggregateId, CorrelationId, CausationId, EventId, Timestamp,
        OrganizationId, UserId, CourseId, EnrollmentId, AssignmentId,
        SubmissionId, MediaId, TenantId, RoleId, PermissionId,
    };
    pub use crate::events::{Event, EventEnvelope, EventMetadata, EventPublisher, EventSubscriber};
    pub use crate::validation::{Validate, ValidationError, ValidationErrors};
    pub use crate::crypto::{Encryptor, Signer, KeyManager, JwtClaims, TokenVerifier};
    pub use crate::observability::{init_tracing, init_metrics, TracingLayer, MetricsLayer};
    pub use crate::messaging::{MessageBus, NatsClient, KafkaClient, MessageSerializer};
    pub use crate::http::{AppState, AuthExtractor, TenantExtractor, RequestId};
    pub use crate::storage::{ObjectStore, DatabasePool, CacheClient};
    pub use crate::errors::{ApexosError, Result, ErrorCode};
    pub use crate::tenancy::{TenantContext, TenantResolver, OrganizationContext};
    pub use crate::rbac::{Permission, Role, Policy, AuthorizationEngine, Subject};
    pub use crate::utils::{generate_id, generate_ulid, now, now_millis, parse_datetime};
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_cargo() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}