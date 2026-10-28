//! Validation
//!
//! Garde-based validation with custom validators for Apexos domain types.

use crate::domain::{Email, PhoneNumber, Locale, MimeType, FileSize, ValidatedUrl};
use crate::errors::{ErrorCode, Result, ValidationError, ValidationErrors};
use garde::{Validate, Report};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use url::Url;
use validator::ValidationErrors as ValidatorErrors;

/// Trait for types that can be validated
pub trait ValidateExt: Validate {
    fn validate_or_error(&self) -> Result<()> {
        self.validate().map_err(|report| {
            let mut errors = ValidationErrors::new();
            for (field, field_errors) in report.iter() {
                for error in field_errors {
                    errors.add(field, error.to_string());
                }
            }
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, "Validation failed")
                .with_context(crate::errors::ErrorContext {
                    fields: errors.0.into_iter().collect(),
                    ..Default::default()
                })
        })
    }
}

impl<T: Validate> ValidateExt for T {}

/// Common validation rules
pub mod rules {
    use super::*;

    /// Validate email format
    pub fn email(email: &str) -> Result<()> {
        Email::new(email).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, e.to_string())
        })
    }

    /// Validate phone number (E.164)
    pub fn phone(phone: &str) -> Result<()> {
        PhoneNumber::new(phone).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, e.to_string())
        })
    }

    /// Validate URL
    pub fn url(url: &str) -> Result<()> {
        ValidatedUrl::parse(url).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, e.to_string())
        })
    }

    /// Validate locale (BCP 47)
    pub fn locale(locale: &str) -> Result<()> {
        // Basic BCP 47 validation
        let parts: Vec<&str> = locale.split('-').collect();
        if parts.is_empty() || parts[0].len() < 2 || parts[0].len() > 3 {
            return Err(crate::errors::ApexosError::new(ErrorCode::ValidationFailed, "Invalid locale format"));
        }
        Ok(())
    }

    /// Validate MIME type
    pub fn mime_type(mime: &str) -> Result<()> {
        mime::Mime::from_str(mime).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, format!("Invalid MIME type: {}", e))
        })
    }

    /// Validate file size within limits
    pub fn file_size(size: u64, max: u64) -> Result<()> {
        if size > max {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                format!("File size {} exceeds maximum {}", size, max),
            ));
        }
        Ok(())
    }

    /// Validate IP address
    pub fn ip_address(ip: &str) -> Result<()> {
        IpAddr::from_str(ip).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, format!("Invalid IP address: {}", e))
        })
    }

    /// Validate non-empty string
    pub fn not_empty(value: &str, field: &str) -> Result<()> {
        if value.trim().is_empty() {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                format!("{} cannot be empty", field),
            ));
        }
        Ok(())
    }

    /// Validate string length
    pub fn length(value: &str, min: usize, max: usize, field: &str) -> Result<()> {
        let len = value.chars().count();
        if len < min || len > max {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                format!("{} must be between {} and {} characters", field, min, max),
            ));
        }
        Ok(())
    }

    /// Validate numeric range
    pub fn range<T: PartialOrd + fmt::Display>(value: T, min: T, max: T, field: &str) -> Result<()> {
        if value < min || value > max {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                format!("{} must be between {} and {}", field, min, max),
            ));
        }
        Ok(())
    }

    /// Validate UUID format
    pub fn uuid(uuid: &str) -> Result<()> {
        uuid::Uuid::from_str(uuid).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, format!("Invalid UUID: {}", e))
        })
    }

    /// Validate ULID format
    pub fn ulid(ulid: &str) -> Result<()> {
        ulid::Ulid::from_str(ulid).map(|_| ()).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, format!("Invalid ULID: {}", e))
        })
    }

    /// Validate slug format
    pub fn slug(slug: &str) -> Result<()> {
        if !slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_') {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                "Slug must contain only lowercase letters, numbers, hyphens, and underscores",
            ));
        }
        if slug.starts_with('-') || slug.ends_with('-') {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                "Slug cannot start or end with hyphen",
            ));
        }
        Ok(())
    }

    /// Validate JSON schema
    pub fn json_schema(value: &serde_json::Value, schema: &schemars::schema::RootSchema) -> Result<()> {
        let validator = jsonschema::Validator::new(schema).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::InternalError, format!("Invalid schema: {}", e))
        })?;
        
        validator.validate(value).map_err(|e| {
            crate::errors::ApexosError::new(ErrorCode::ValidationFailed, format!("Schema validation failed: {}", e))
        })?;
        Ok(())
    }
}

/// Validation context for cross-field validation
#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    pub tenant_id: Option<crate::domain::TenantId>,
    pub user_id: Option<crate::domain::UserId>,
    pub organization_id: Option<crate::domain::OrganizationId>,
    pub custom: HashMap<String, serde_json::Value>,
}

/// Async validator trait for database-dependent validations
#[async_trait::async_trait]
pub trait AsyncValidator<T> {
    async fn validate_async(&self, value: &T, context: &ValidationContext) -> Result<()>;
}

/// Password strength validator
pub struct PasswordValidator {
    pub config: crate::config::PasswordConfig,
}

impl PasswordValidator {
    pub fn new(config: crate::config::PasswordConfig) -> Self {
        Self { config }
    }

    pub fn validate(&self, password: &str, personal_info: Option<&[String]>) -> Result<()> {
        let mut errors = Vec::new();

        // Length checks
        if password.len() < self.config.min_length {
            errors.push(format!("Password must be at least {} characters", self.config.min_length));
        }
        if password.len() > self.config.max_length {
            errors.push(format!("Password must not exceed {} characters", self.config.max_length));
        }

        // Character requirements
        if self.config.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }
        if self.config.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }
        if self.config.require_numbers && !password.chars().any(|c| c.is_ascii_digit()) {
            errors.push("Password must contain at least one number".to_string());
        }
        if self.config.require_symbols && !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            errors.push("Password must contain at least one symbol".to_string());
        }

        // Common password check
        if self.config.forbid_common && is_common_password(password) {
            errors.push("Password is too common".to_string());
        }

        // Personal info check
        if self.config.forbid_personal_info {
            if let Some(info) = personal_info {
                for item in info {
                    if password.to_lowercase().contains(&item.to_lowercase()) {
                        errors.push("Password cannot contain personal information".to_string());
                        break;
                    }
                }
            }
        }

        if !errors.is_empty() {
            return Err(crate::errors::ApexosError::new(
                ErrorCode::ValidationFailed,
                errors.join("; "),
            ));
        }

        Ok(())
    }
}

fn is_common_password(password: &str) -> bool {
    // In production, use a proper password breached database
    const COMMON: &[&str] = &[
        "password", "123456", "123456789", "qwerty", "abc123",
        "password123", "admin", "letmein", "welcome", "monkey",
    ];
    COMMON.iter().any(|&p| p.eq_ignore_ascii_case(password))
}

/// Unique constraint validator (for database checks)
pub struct UniqueValidator {
    pub table: String,
    pub column: String,
    pub exclude_id: Option<crate::domain::AggregateId>,
}

#[async_trait::async_trait]
impl AsyncValidator<String> for UniqueValidator {
    async fn validate_async(&self, value: &String, context: &ValidationContext) -> Result<()> {
        // This would typically query the database
        // Implementation depends on the specific service
        // Placeholder for now
        Ok(())
    }
}

/// Slug uniqueness validator
pub struct SlugValidator {
    pub table: String,
    pub column: String,
    pub scope_column: Option<String>, // e.g., "organization_id"
}

#[async_trait::async_trait]
impl AsyncValidator<String> for SlugValidator {
    async fn validate_async(&self, value: &String, context: &ValidationContext) -> Result<()> {
        rules::slug(value)?;
        // Database uniqueness check would go here
        Ok(())
    }
}

/// Email uniqueness validator
pub struct EmailValidator {
    pub exclude_user_id: Option<crate::domain::UserId>,
}

#[async_trait::async_trait]
impl AsyncValidator<String> for EmailValidator {
    async fn validate_async(&self, value: &String, context: &ValidationContext) -> Result<()> {
        rules::email(value)?;
        // Database uniqueness check would go here
        Ok(())
    }
}

/// Domain validation helper
pub struct DomainValidator;

impl DomainValidator {
    /// Validate course enrollment eligibility
    pub async fn validate_enrollment(
        user_id: UserId,
        course_id: CourseId,
        context: &ValidationContext,
    ) -> Result<()> {
        // Check prerequisites, capacity, enrollment dates, etc.
        // Implementation in web-app service
        Ok(())
    }

    /// Validate assignment submission eligibility
    pub async fn validate_submission(
        user_id: UserId,
        assignment_id: AssignmentId,
        context: &ValidationContext,
    ) -> Result<()> {
        // Check enrollment, due date, attempts, etc.
        Ok(())
    }

    /// Validate grade submission
    pub async fn validate_grade(
        grader_id: UserId,
        submission_id: SubmissionId,
        context: &ValidationContext,
    ) -> Result<()> {
        // Check permissions, submission status, etc.
        Ok(())
    }
}

/// Convert validator crate errors to our format
pub fn convert_validator_errors(errors: ValidatorErrors) -> ValidationErrors {
    let mut validation_errors = ValidationErrors::new();
    for (field, field_errors) in errors.errors() {
        for error in field_errors {
            validation_errors.add(field, error.message.unwrap_or_default().to_string());
        }
    }
    validation_errors
}

/// Convert garde report to our format
pub fn convert_garde_report(report: Report) -> ValidationErrors {
    let mut validation_errors = ValidationErrors::new();
    for (field, field_errors) in report.iter() {
        for error in field_errors {
            validation_errors.add(field, error.to_string());
        }
    }
    validation_errors
}

/// Validation result wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: ValidationErrors,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            valid: true,
            errors: ValidationErrors::new(),
            warnings: Vec::new(),
        }
    }

    pub fn invalid(errors: ValidationErrors) -> Self {
        Self {
            valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }

    pub fn add_error(&mut self, field: &str, message: &str) {
        self.valid = false;
        self.errors.add(field, message);
    }

    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    pub fn into_result(self) -> Result<()> {
        if self.valid {
            Ok(())
        } else {
            Err(crate::errors::ApexosError::new(ErrorCode::ValidationFailed, "Validation failed")
                .with_context(crate::errors::ErrorContext {
                    fields: self.errors.0.into_iter().collect(),
                    ..Default::default()
                }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    #[test]
    fn email_validation() {
        assert!(rules::email("test@example.com").is_ok());
        assert!(rules::email("invalid").is_err());
        assert!(rules::email("test@").is_err());
        assert!(rules::email("@example.com").is_err());
    }

    #[test]
    fn phone_validation() {
        assert!(rules::phone("+15551234567").is_ok());
        assert!(rules::phone("5551234567").is_err());
    }

    #[test]
    fn url_validation() {
        assert!(rules::url("https://example.com").is_ok());
        assert!(rules::url("not-a-url").is_err());
    }

    #[test]
    fn slug_validation() {
        assert!(rules::slug("valid-slug").is_ok());
        assert!(rules::slug("valid_slug").is_ok());
        assert!(rules::slug("valid123").is_ok());
        assert!(rules::slug("-invalid").is_err());
        assert!(rules::slug("invalid-").is_err());
        assert!(rules::slug("Invalid").is_err());
    }

    #[test]
    fn password_validator() {
        let config = crate::config::PasswordConfig {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_symbols: true,
            forbid_common: true,
            forbid_personal_info: true,
            history_count: 5,
            max_age: None,
            argon2: crate::config::Argon2Config {
                variant: crate::config::Argon2Variant::Argon2id,
                version: 19,
                memory_cost: 65536,
                time_cost: 3,
                parallelism: 4,
                hash_length: 32,
                salt_length: 16,
            },
        };

        let validator = PasswordValidator::new(config);
        
        assert!(validator.validate("StrongPass123!", None).is_ok());
        assert!(validator.validate("weak", None).is_err());
        assert!(validator.validate("NoNumbers!", None).is_err());
        assert!(validator.validate("nonumbers1", None).is_err());
        assert!(validator.validate("NONUMBERS1!", None).is_err());
    }
}