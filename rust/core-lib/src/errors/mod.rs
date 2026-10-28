//! Error Types
//!
//! Standardized error handling across all Apexos services with error codes,
//! structured context, and proper HTTP status mapping.

use crate::domain::{AggregateId, CorrelationId, EventId, TenantId, UserId};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tracing::error;

/// Standardized error codes for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    // Generic
    InternalError,
    InvalidRequest,
    ValidationFailed,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    ServiceUnavailable,
    Timeout,
    PayloadTooLarge,
    UnsupportedMediaType,

    // Authentication
    InvalidCredentials,
    TokenExpired,
    TokenInvalid,
    TokenRevoked,
    MfaRequired,
    MfaInvalid,
    AccountLocked,
    AccountDisabled,
    PasswordExpired,
    WeakPassword,
    EmailNotVerified,
    PhoneNotVerified,

    // Authorization
    InsufficientPermissions,
    ResourceAccessDenied,
    OrganizationAccessDenied,
    TenantAccessDenied,
    RoleAssignmentFailed,

    // Tenancy
    TenantNotFound,
    TenantSuspended,
    TenantQuotaExceeded,
    TenantDomainConflict,
    SubdomainReserved,

    // Domain-specific
    OrganizationNotFound,
    UserNotFound,
    UserAlreadyExists,
    CourseNotFound,
    CourseNotPublished,
    EnrollmentNotFound,
    EnrollmentClosed,
    EnrollmentFull,
    PrerequisiteNotMet,
    AssignmentNotFound,
    AssignmentNotOpen,
    SubmissionNotFound,
    SubmissionClosed,
    GradeNotFound,
    MediaNotFound,
    MediaProcessingFailed,
    MediaTooLarge,
    MediaTypeNotSupported,
    SearchIndexUnavailable,
    SearchQueryInvalid,
    RealtimeConnectionFailed,
    RealtimeMessageTooLarge,
    SchedulerJobFailed,
    WorkflowNotFound,
    WorkflowStepFailed,
    CertificateNotFound,
    CertificateRevoked,
    BadgeNotFound,
    LearningPathNotFound,
    CompetencyNotFound,
    PlagiarismDetected,
    LTILaunchFailed,
    LTIInvalidState,
    LTIToolNotConfigured,
    SCORMNotFound,
    SCORMSequencingError,
    XAPIInvalidStatement,
    CaliperInvalidEvent,
    OneRosterSyncFailed,
    QTIInvalidAssessment,
    QTIInvalidItem,

    // Integrations
    ExternalServiceError,
    WebhookDeliveryFailed,
    WebhookSignatureInvalid,
    S3OperationFailed,
    DatabaseError,
    CacheError,
    MessageQueueError,
    EmailDeliveryFailed,
    SmsDeliveryFailed,
    PushNotificationFailed,

    // AI/ML
    ModelNotFound,
    ModelUnavailable,
    InferenceFailed,
    EmbeddingFailed,
    TrainingFailed,
    DatasetNotFound,
    FeatureStoreError,
    RecommendationFailed,
    GradingFailed,
    AnalyticsQueryFailed,
}

impl ErrorCode {
    /// HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        use ErrorCode::*;
        match self {
            // 4xx
            InvalidRequest | ValidationFailed | PayloadTooLarge | UnsupportedMediaType => {
                StatusCode::BAD_REQUEST
            }
            Unauthorized
            | InvalidCredentials
            | TokenExpired
            | TokenInvalid
            | TokenRevoked
            | MfaRequired
            | MfaInvalid
            | EmailNotVerified
            | PhoneNotVerified => StatusCode::UNAUTHORIZED,
            Forbidden
            | InsufficientPermissions
            | ResourceAccessDenied
            | OrganizationAccessDenied
            | TenantAccessDenied
            | AccountDisabled
            | AccountLocked => StatusCode::FORBIDDEN,
            NotFound
            | TenantNotFound
            | OrganizationNotFound
            | UserNotFound
            | CourseNotFound
            | EnrollmentNotFound
            | AssignmentNotFound
            | SubmissionNotFound
            | GradeNotFound
            | MediaNotFound
            | WorkflowNotFound
            | CertificateNotFound
            | BadgeNotFound
            | LearningPathNotFound
            | CompetencyNotFound
            | ModelNotFound
            | DatasetNotFound
            | LTIToolNotConfigured
            | SCORMNotFound
            | QTIAssessmentNotFound
            | QTIItemNotFound => StatusCode::NOT_FOUND,
            Conflict
            | UserAlreadyExists
            | TenantDomainConflict
            | SubdomainReserved
            | EnrollmentFull => StatusCode::CONFLICT,
            RateLimited => StatusCode::TOO_MANY_REQUESTS,
            CourseNotPublished
            | EnrollmentClosed
            | PrerequisiteNotMet
            | AssignmentNotOpen
            | SubmissionClosed
            | MediaProcessingFailed
            | MediaTooLarge
            | MediaTypeNotSupported
            | SearchQueryInvalid
            | RealtimeMessageTooLarge
            | LTIInvalidState
            | SCORMSequencingError
            | XAPIInvalidStatement
            | CaliperInvalidEvent
            | QTIInvalidAssessment
            | QTIInvalidItem
            | WeakPassword
            | PasswordExpired => StatusCode::UNPROCESSABLE_ENTITY,

            // 5xx
            InternalError
            | ServiceUnavailable
            | Timeout
            | SearchIndexUnavailable
            | RealtimeConnectionFailed
            | SchedulerJobFailed
            | WorkflowStepFailed
            | ExternalServiceError
            | WebhookDeliveryFailed
            | S3OperationFailed
            | DatabaseError
            | CacheError
            | MessageQueueError
            | EmailDeliveryFailed
            | SmsDeliveryFailed
            | PushNotificationFailed
            | ModelUnavailable
            | InferenceFailed
            | EmbeddingFailed
            | TrainingFailed
            | FeatureStoreError
            | RecommendationFailed
            | GradingFailed
            | AnalyticsQueryFailed
            | OneRosterSyncFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Human-readable message
    pub fn message(&self) -> &'static str {
        use ErrorCode::*;
        match self {
            InternalError => "An internal server error occurred",
            InvalidRequest => "The request is invalid or malformed",
            ValidationFailed => "Request validation failed",
            Unauthorized => "Authentication required",
            Forbidden => "Access denied",
            NotFound => "The requested resource was not found",
            Conflict => "The request conflicts with the current state",
            RateLimited => "Too many requests, please try again later",
            ServiceUnavailable => "Service temporarily unavailable",
            Timeout => "The request timed out",
            PayloadTooLarge => "Request payload too large",
            UnsupportedMediaType => "Unsupported media type",

            InvalidCredentials => "Invalid credentials provided",
            TokenExpired => "Authentication token has expired",
            TokenInvalid => "Authentication token is invalid",
            TokenRevoked => "Authentication token has been revoked",
            MfaRequired => "Multi-factor authentication required",
            MfaInvalid => "Invalid multi-factor authentication code",
            AccountLocked => "Account has been locked",
            AccountDisabled => "Account has been disabled",
            PasswordExpired => "Password has expired",
            WeakPassword => "Password does not meet requirements",
            EmailNotVerified => "Email address not verified",
            PhoneNotVerified => "Phone number not verified",

            InsufficientPermissions => "Insufficient permissions for this action",
            ResourceAccessDenied => "Access to this resource is denied",
            OrganizationAccessDenied => "Access to this organization is denied",
            TenantAccessDenied => "Access to this tenant is denied",
            RoleAssignmentFailed => "Failed to assign role",

            TenantNotFound => "Tenant not found",
            TenantSuspended => "Tenant has been suspended",
            TenantQuotaExceeded => "Tenant quota exceeded",
            TenantDomainConflict => "Domain already in use",
            SubdomainReserved => "Subdomain is reserved",

            OrganizationNotFound => "Organization not found",
            UserNotFound => "User not found",
            UserAlreadyExists => "User already exists",
            CourseNotFound => "Course not found",
            CourseNotPublished => "Course is not published",
            EnrollmentNotFound => "Enrollment not found",
            EnrollmentClosed => "Enrollment is closed",
            EnrollmentFull => "Course is full",
            PrerequisiteNotMet => "Prerequisites not met",
            AssignmentNotFound => "Assignment not found",
            AssignmentNotOpen => "Assignment is not open",
            SubmissionNotFound => "Submission not found",
            SubmissionClosed => "Submission period closed",
            GradeNotFound => "Grade not found",
            MediaNotFound => "Media not found",
            MediaProcessingFailed => "Media processing failed",
            MediaTooLarge => "Media file too large",
            MediaTypeNotSupported => "Media type not supported",
            SearchIndexUnavailable => "Search index unavailable",
            SearchQueryInvalid => "Invalid search query",
            RealtimeConnectionFailed => "Real-time connection failed",
            RealtimeMessageTooLarge => "Real-time message too large",
            SchedulerJobFailed => "Scheduled job failed",
            WorkflowNotFound => "Workflow not found",
            WorkflowStepFailed => "Workflow step failed",
            CertificateNotFound => "Certificate not found",
            CertificateRevoked => "Certificate has been revoked",
            BadgeNotFound => "Badge not found",
            LearningPathNotFound => "Learning path not found",
            CompetencyNotFound => "Competency not found",
            PlagiarismDetected => "Plagiarism detected",
            LTILaunchFailed => "LTI launch failed",
            LTIInvalidState => "Invalid LTI state",
            LTIToolNotConfigured => "LTI tool not configured",
            SCORMNotFound => "SCORM package not found",
            SCORMSequencingError => "SCORM sequencing error",
            XAPIInvalidStatement => "Invalid xAPI statement",
            CaliperInvalidEvent => "Invalid Caliper event",
            OneRosterSyncFailed => "OneRoster sync failed",
            QTIInvalidAssessment => "Invalid QTI assessment",
            QTIInvalidItem => "Invalid QTI item",

            ExternalServiceError => "External service error",
            WebhookDeliveryFailed => "Webhook delivery failed",
            WebhookSignatureInvalid => "Invalid webhook signature",
            S3OperationFailed => "Storage operation failed",
            DatabaseError => "Database error",
            CacheError => "Cache error",
            MessageQueueError => "Message queue error",
            EmailDeliveryFailed => "Email delivery failed",
            SmsDeliveryFailed => "SMS delivery failed",
            PushNotificationFailed => "Push notification failed",

            ModelNotFound => "AI model not found",
            ModelUnavailable => "AI model unavailable",
            InferenceFailed => "AI inference failed",
            EmbeddingFailed => "Embedding generation failed",
            TrainingFailed => "Model training failed",
            DatasetNotFound => "Dataset not found",
            FeatureStoreError => "Feature store error",
            RecommendationFailed => "Recommendation generation failed",
            GradingFailed => "Auto-grading failed",
            AnalyticsQueryFailed => "Analytics query failed",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

/// Main error type for Apexos services
#[derive(Debug, Error)]
#[error("{code}: {message}")]
pub struct ApexosError {
    pub code: ErrorCode,
    pub message: String,
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
    pub context: ErrorContext,
    pub correlation_id: Option<CorrelationId>,
    pub event_id: Option<EventId>,
}

impl ApexosError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            source: None,
            context: ErrorContext::default(),
            correlation_id: None,
            event_id: None,
        }
    }

    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.event_id = Some(event_id);
        self
    }

    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.fields.insert(key.into(), value.into());
        self
    }

    pub fn with_user(mut self, user_id: UserId) -> Self {
        self.context.user_id = Some(user_id);
        self
    }

    pub fn with_tenant(mut self, tenant_id: TenantId) -> Self {
        self.context.tenant_id = Some(tenant_id);
        self
    }

    pub fn with_organization(mut self, org_id: crate::domain::OrganizationId) -> Self {
        self.context.organization_id = Some(org_id);
        self
    }

    pub fn with_resource(mut self, resource_type: impl Into<String>, resource_id: AggregateId) -> Self {
        self.context.resource_type = Some(resource_type.into());
        self.context.resource_id = Some(resource_id);
        self
    }

    /// Convert to API error response
    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: ErrorDetail {
                code: self.code,
                message: self.message.clone(),
                correlation_id: self.correlation_id,
                event_id: self.event_id,
                context: if self.context.is_empty() {
                    None
                } else {
                    Some(self.context.clone())
                },
            },
        }
    }
}

/// Context attached to errors for debugging and observability
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorContext {
    pub user_id: Option<UserId>,
    pub tenant_id: Option<TenantId>,
    pub organization_id: Option<crate::domain::OrganizationId>,
    pub resource_type: Option<String>,
    pub resource_id: Option<AggregateId>,
    pub fields: HashMap<String, String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

impl ErrorContext {
    pub fn is_empty(&self) -> bool {
        self.user_id.is_none()
            && self.tenant_id.is_none()
            && self.organization_id.is_none()
            && self.resource_type.is_none()
            && self.resource_id.is_none()
            && self.fields.is_empty()
            && self.trace_id.is_none()
            && self.span_id.is_none()
    }
}

/// API error response format (RFC 7807 compatible)
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<CorrelationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
}

/// Result type alias
pub type Result<T> = std::result::Result<T, ApexosError>;

/// Convert any error to ApexosError with context
pub trait IntoApexosError<T> {
    fn into_apexos_error(self, code: ErrorCode) -> Result<T>;
    fn into_apexos_error_with_context(self, code: ErrorCode, context: ErrorContext) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> IntoApexosError<T> for std::result::Result<T, E> {
    fn into_apexos_error(self, code: ErrorCode) -> Result<T> {
        self.map_err(|e| ApexosError::new(code, e.to_string()).with_source(e))
    }

    fn into_apexos_error_with_context(self, code: ErrorCode, context: ErrorContext) -> Result<T> {
        self.map_err(|e| ApexosError::new(code, e.to_string()).with_source(e).with_context(context))
    }
}

/// Axum IntoResponse implementation for ApexosError
impl IntoResponse for ApexosError {
    fn into_response(self) -> Response {
        let status = self.code.status_code();
        let response = self.to_response();

        // Log error with context
        error!(
            error_code = %self.code,
            message = %self.message,
            correlation_id = ?self.correlation_id,
            event_id = ?self.event_id,
            user_id = ?self.context.user_id,
            tenant_id = ?self.context.tenant_id,
            organization_id = ?self.context.organization_id,
            resource_type = ?self.context.resource_type,
            resource_id = ?self.context.resource_id,
            fields = ?self.context.fields,
            "error response"
        );

        (status, Json(response)).into_response()
    }
}

/// Convert from common error types
impl From<sqlx::Error> for ApexosError {
    fn from(e: sqlx::Error) -> Self {
        use sqlx::Error::*;
        match e {
            RowNotFound => ApexosError::new(ErrorCode::NotFound, "Resource not found"),
            ColumnNotFound(_) => ApexosError::new(ErrorCode::InternalError, "Database schema error"),
            _ => ApexosError::new(ErrorCode::DatabaseError, e.to_string()).with_source(e),
        }
    }
}

impl From<redis::RedisError> for ApexosError {
    fn from(e: redis::RedisError) -> Self {
        ApexosError::new(ErrorCode::CacheError, e.to_string()).with_source(e)
    }
}

impl From<async_nats::Error> for ApexosError {
    fn from(e: async_nats::Error) -> Self {
        ApexosError::new(ErrorCode::MessageQueueError, e.to_string()).with_source(e)
    }
}

impl From<rdkafka::error::KafkaError> for ApexosError {
    fn from(e: rdkafka::error::KafkaError) -> Self {
        ApexosError::new(ErrorCode::MessageQueueError, e.to_string()).with_source(e)
    }
}

impl From<aws_sdk_s3::Error> for ApexosError {
    fn from(e: aws_sdk_s3::Error) -> Self {
        ApexosError::new(ErrorCode::S3OperationFailed, e.to_string()).with_source(e)
    }
}

impl From<jsonwebtoken::errors::Error> for ApexosError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        ApexosError::new(ErrorCode::TokenInvalid, e.to_string()).with_source(e)
    }
}

impl From<oauth2::basic::BasicErrorResponse> for ApexosError {
    fn from(e: oauth2::basic::BasicErrorResponse) -> Self {
        ApexosError::new(ErrorCode::InvalidCredentials, e.to_string())
    }
}

impl From<validator::ValidationErrors> for ApexosError {
    fn from(e: validator::ValidationErrors) -> Self {
        ApexosError::new(ErrorCode::ValidationFailed, e.to_string()).with_source(e)
    }
}

impl From<garde::Report> for ApexosError {
    fn from(e: garde::Report) -> Self {
        ApexosError::new(ErrorCode::ValidationFailed, e.to_string())
    }
}

impl From<serde_json::Error> for ApexosError {
    fn from(e: serde_json::Error) -> Self {
        ApexosError::new(ErrorCode::InvalidRequest, e.to_string()).with_source(e)
    }
}

impl From<std::io::Error> for ApexosError {
    fn from(e: std::io::Error) -> Self {
        ApexosError::new(ErrorCode::InternalError, e.to_string()).with_source(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ApexosError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ApexosError::new(ErrorCode::InternalError, e.to_string()).with_source(e)
    }
}

/// Helper macros for common error patterns
#[macro_export]
macro_rules! not_found {
    ($resource:expr, $id:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::NotFound, format!("{} not found: {}", $resource, $id))
            .with_resource(stringify!($resource), $id)
    };
}

#[macro_export]
macro_rules! unauthorized {
    ($msg:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::Unauthorized, $msg)
    };
    () => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::Unauthorized, "Authentication required")
    };
}

#[macro_export]
macro_rules! forbidden {
    ($msg:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::Forbidden, $msg)
    };
    () => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::Forbidden, "Access denied")
    };
}

#[macro_export]
macro_rules! validation_error {
    ($msg:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::ValidationFailed, $msg)
    };
}

#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::InternalError, $msg)
    };
    ($err:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::InternalError, $err.to_string()).with_source($err)
    };
}

#[macro_export]
macro_rules! conflict {
    ($msg:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::Conflict, $msg)
    };
}

#[macro_export]
macro_rules! rate_limited {
    ($retry_after:expr) => {
        $crate::errors::ApexosError::new($crate::errors::ErrorCode::RateLimited, "Rate limited")
            .with_field("retry_after", $retry_after.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_status_mapping() {
        assert_eq!(ErrorCode::NotFound.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(ErrorCode::Unauthorized.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ErrorCode::Forbidden.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(ErrorCode::RateLimited.status_code(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(ErrorCode::InternalError.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn error_builder() {
        let err = ApexosError::new(ErrorCode::NotFound, "User not found")
            .with_user(UserId::new())
            .with_tenant(TenantId::new())
            .with_field("key", "value");

        assert_eq!(err.code, ErrorCode::NotFound);
        assert!(err.context.user_id.is_some());
        assert!(err.context.tenant_id.is_some());
        assert_eq!(err.context.fields.get("key"), Some(&"value".to_string()));
    }
}