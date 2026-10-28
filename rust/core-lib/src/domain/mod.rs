//! Domain Types
//!
//! Core identifiers, value objects, and aggregate roots used across all services.
//! These types ensure type safety and semantic meaning throughout the system.

use crate::utils::{generate_id, generate_ulid, now, now_millis};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use ulid::Ulid;
use uuid::Uuid;

/// Macro to create newtype identifiers with standard traits
macro_rules! define_id {
    ($name:ident, $prefix:literal) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
            Serialize, Deserialize
        )]
        #[serde(transparent)]
        pub struct $name(pub Ulid);

        impl $name {
            /// Generate a new unique identifier
            pub fn new() -> Self {
                Self(Ulid::new())
            }

            /// Create from a raw ULID
            pub fn from_ulid(ulid: Ulid) -> Self {
                Self(ulid)
            }

            /// Create from a UUID v7 (timestamp-based)
            pub fn from_uuid_v7(uuid: Uuid) -> Self {
                Self(Ulid::from_uuid(uuid))
            }

            /// Get the inner ULID
            pub fn as_ulid(&self) -> Ulid {
                self.0
            }

            /// Get the timestamp component
            pub fn timestamp(&self) -> i64 {
                self.0.datetime().timestamp_millis()
            }

            /// Get the prefix for display
            pub const fn prefix() -> &'static str {
                $prefix
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}_{}", $prefix, self.0)
            }
        }

        impl Deref for $name {
            type Target = Ulid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<Ulid> for $name {
            fn from(ulid: Ulid) -> Self {
                Self(ulid)
            }
        }

        impl From<$name> for Ulid {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.to_string()
            }
        }

        impl std::str::FromStr for $name {
            type Err = UlidError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // Handle both "prefix_ulid" and bare ULID formats
                let ulid_str = s.strip_prefix(concat!($prefix, "_")).unwrap_or(s);
                Ok(Self(Ulid::from_str(ulid_str)?))
            }
        }

        /// Error type for ID parsing
        #[derive(Debug, thiserror::Error)]
        #[error("invalid {0} ID: {1}")]
        pub struct UlidError(pub &'static str, pub String);
    };
}

// Core aggregate identifiers
define_id!(AggregateId, "agg");
define_id!(EventId, "evt");
define_id!(CorrelationId, "corr");
define_id!(CausationId, "cause");
define_id!(TenantId, "ten");
define_id!(OrganizationId, "org");
define_id!(UserId, "usr");
define_id!(CourseId, "crs");
define_id!(EnrollmentId, "enr");
define_id!(AssignmentId, "asg");
define_id!(SubmissionId, "sub");
define_id!(MediaId, "med");
define_id!(RoleId, "rol");
define_id!(PermissionId, "perm");
define_id!(SessionId, "ses");
define_id!(ApiKeyId, "key");
define_id!(WebhookId, "whk");
define_id!(NotificationId, "notif");
define_id!(AuditLogId, "aud");
define_id!(JobId, "job");
define_id!(WorkflowId, "wf");
define_id!(StepId, "step");
define_id!(ArtifactId, "art");
define_id!(CertificateId, "cert");
define_id!(BadgeId, "bdg");
define_id!(LearningPathId, "lp");
define_id!(CompetencyId, "comp");
define_id!(QuestionId, "qst");
define_id!(AttemptId, "atmpt");
define_id!(DiscussionId, "dsc");
define_id!(PostId, "pst");
define_id!(CommentId, "cmt");
define_id!(GroupId, "grp");
define_id!(MembershipId, "mem");
define_id!(CalendarEventId, "cal");
define_id!(AnnouncementId, "ann");
define_id!(SurveyId, "srv");
define_id!(ResponseId, "rsp");
define_id!(RubricId, "rub");
define_id!(CriterionId, "crit");
define_id!(GradeId, "grd");
define_id!(FeedbackId, "fbk");
define_id!(PlagiarismReportId, "plag");
define_id!(LTILaunchId, "lti");
define_id!(SCOMPackageId, "scorm");
define_id!(XAPIStatementId, "xapi");
define_id!(CaliperEventId, "calip");
define_id!(OneRosterId, "onr");
define_id!(QTIAssessmentId, "qti");
define_id!(QTIItemId, "qtiitem");

/// Timestamp wrapper with serialization support
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(pub i64);

impl Timestamp {
    /// Current timestamp in milliseconds since Unix epoch
    pub fn now() -> Self {
        Self(now_millis())
    }

    /// Create from milliseconds since Unix epoch
    pub const fn from_millis(millis: i64) -> Self {
        Self(millis)
    }

    /// Create from seconds since Unix epoch
    pub const fn from_secs(secs: i64) -> Self {
        Self(secs * 1000)
    }

    /// Get milliseconds since Unix epoch
    pub const fn as_millis(&self) -> i64 {
        self.0
    }

    /// Get seconds since Unix epoch
    pub const fn as_secs(&self) -> i64 {
        self.0 / 1000
    }

    /// Convert to chrono DateTime<Utc>
    pub fn to_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp_millis(self.0).unwrap_or_default()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_datetime().to_rfc3339())
    }
}

impl From<i64> for Timestamp {
    fn from(millis: i64) -> Self {
        Self(millis)
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt.timestamp_millis())
    }
}

/// Composite key for tenant-scoped entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TenantScopedId<T> {
    pub tenant_id: TenantId,
    pub entity_id: T,
}

impl<T> TenantScopedId<T> {
    pub fn new(tenant_id: TenantId, entity_id: T) -> Self {
        Self { tenant_id, entity_id }
    }
}

impl<T: fmt::Display> fmt::Display for TenantScopedId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.tenant_id, self.entity_id)
    }
}

/// Version for optimistic locking
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Version(pub u64);

impl Version {
    pub const fn new() -> Self {
        Self(1)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Soft delete marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletedAt(pub Option<Timestamp>);

impl DeletedAt {
    pub const fn alive() -> Self {
        Self(None)
    }

    pub fn deleted(at: Timestamp) -> Self {
        Self(Some(at))
    }

    pub fn is_deleted(&self) -> bool {
        self.0.is_some()
    }

    pub fn as_option(&self) -> Option<Timestamp> {
        self.0
    }
}

impl Default for DeletedAt {
    fn default() -> Self {
        Self::alive()
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
}

impl Pagination {
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 1000),
        }
    }

    pub fn offset(&self) -> u64 {
        ((self.page - 1) as u64) * (self.per_page as u64)
    }

    pub fn limit(&self) -> u64 {
        self.per_page as u64
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new(1, 50)
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub per_page: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> Paginated<T> {
    pub fn new(data: Vec<T>, pagination: Pagination, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (pagination.per_page as f64)).ceil() as u32;
        Self {
            data,
            pagination: PaginationMeta {
                current_page: pagination.page,
                per_page: pagination.per_page,
                total_items,
                total_pages,
                has_next: pagination.page < total_pages,
                has_prev: pagination.page > 1,
            },
        }
    }
}

/// Sort parameters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sort {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}

/// Filter operators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
    IsNull,
    IsNotNull,
    Between,
}

/// Filter condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

/// Query parameters for list endpoints
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListQuery {
    pub pagination: Pagination,
    pub sort: Option<Sort>,
    pub filters: Vec<Filter>,
    pub search: Option<String>,
    pub include: Vec<String>,
}

/// Language/locale code (BCP 47)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locale(pub String);

impl Locale {
    pub const EN_US: &'static str = "en-US";
    pub const EN_GB: &'static str = "en-GB";
    pub const ES_ES: &'static str = "es-ES";
    pub const FR_FR: &'static str = "fr-FR";
    pub const DE_DE: &'static str = "de-DE";
    pub const ZH_CN: &'static str = "zh-CN";
    pub const JA_JP: &'static str = "ja-JP";
    pub const PT_BR: &'static str = "pt-BR";
    pub const AR_SA: &'static str = "ar-SA";
    pub const HI_IN: &'static str = "hi-IN";
    pub const RU_RU: &'static str = "ru-RU";

    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    pub fn language(&self) -> &str {
        self.0.split('-').next().unwrap_or(&self.0)
    }

    pub fn region(&self) -> Option<&str> {
        self.0.split('-').nth(1)
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::new(Self::EN_US)
    }
}

/// MIME type wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MimeType(pub String);

impl MimeType {
    pub fn new(mime: impl Into<String>) -> Self {
        Self(mime.into())
    }

    pub fn is_image(&self) -> bool {
        self.0.starts_with("image/")
    }

    pub fn is_video(&self) -> bool {
        self.0.starts_with("video/")
    }

    pub fn is_audio(&self) -> bool {
        self.0.starts_with("audio/")
    }

    pub fn is_document(&self) -> bool {
        matches!(
            self.0.as_str(),
            "application/pdf"
                | "application/msword"
                | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                | "application/vnd.ms-excel"
                | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                | "application/vnd.ms-powerpoint"
                | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                | "text/plain"
                | "text/markdown"
                | "text/csv"
        )
    }

    pub fn is_scorm(&self) -> bool {
        self.0 == "application/vnd.adobe.scorm" || self.0 == "application/zip"
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<mime::Mime> for MimeType {
    fn from(m: mime::Mime) -> Self {
        Self(m.to_string())
    }
}

impl From<MimeType> for mime::Mime {
    fn from(m: MimeType) -> Self {
        m.0.parse().unwrap_or(mime::APPLICATION_OCTET_STREAM)
    }
}

/// File size with human-readable formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileSize(pub u64);

impl FileSize {
    pub const fn bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    pub const fn kb(kb: u64) -> Self {
        Self(kb * 1_024)
    }

    pub const fn mb(mb: u64) -> Self {
        Self(mb * 1_024 * 1_024)
    }

    pub const fn gb(gb: u64) -> Self {
        Self(gb * 1_024 * 1_024 * 1_024)
    }

    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    pub fn as_kb(&self) -> f64 {
        self.0 as f64 / 1_024.0
    }

    pub fn as_mb(&self) -> f64 {
        self.0 as f64 / (1_024.0 * 1_024.0)
    }

    pub fn as_gb(&self) -> f64 {
        self.0 as f64 / (1_024.0 * 1_024.0 * 1_024.0)
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
        let mut size = self.0 as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        if unit_idx == 0 {
            write!(f, "{} {}", self.0, UNITS[unit_idx])
        } else {
            write!(f, "{:.2} {}", size, UNITS[unit_idx])
        }
    }
}

impl From<u64> for FileSize {
    fn from(bytes: u64) -> Self {
        Self(bytes)
    }
}

impl From<FileSize> for u64 {
    fn from(size: FileSize) -> Self {
        size.0
    }
}

/// URL with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidatedUrl(pub url::Url);

impl ValidatedUrl {
    pub fn parse(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self(url.parse()?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for ValidatedUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ValidatedUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<url::Url> for ValidatedUrl {
    fn from(url: url::Url) -> Self {
        Self(url)
    }
}

impl From<ValidatedUrl> for url::Url {
    fn from(url: ValidatedUrl) -> Self {
        url.0
    }
}

/// Email address with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into().to_lowercase();
        if !validator::validate_email(&email) {
            return Err(EmailError(email));
        }
        Ok(Self(email))
    }

    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }

    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid email address: {0}")]
pub struct EmailError(pub String);

/// Phone number with validation (E.164 format)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    pub fn new(phone: impl Into<String>) -> Result<Self, PhoneError> {
        let phone = phone.into();
        let parsed = phonenumbers::PhoneNumber::parse(&phone, None)
            .map_err(|_| PhoneError(phone.clone()))?;

        if !phonenumbers::is_valid_phone_number(&parsed) {
            return Err(PhoneError(phone));
        }

        Ok(Self(phonenumbers::format_number(
            &parsed,
            phonenumbers::PhoneNumberFormat::E164,
        )))
    }

    pub fn national_format(&self) -> String {
        let parsed = phonenumbers::PhoneNumber::parse(&self.0, None).unwrap();
        phonenumbers::format_number(&parsed, phonenumbers::PhoneNumberFormat::National)
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for PhoneNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid phone number: {0}")]
pub struct PhoneError(pub String);

/// IP address wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpAddr(pub std::net::IpAddr);

impl IpAddr {
    pub fn is_private(&self) -> bool {
        match self.0 {
            std::net::IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
            std::net::IpAddr::V6(v6) => v6.is_loopback() || v6.is_unique_local(),
        }
    }

    pub fn is_loopback(&self) -> bool {
        self.0.is_loopback()
    }
}

impl Deref for IpAddr {
    type Target = std::net::IpAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::net::IpAddr> for IpAddr {
    fn from(ip: std::net::IpAddr) -> Self {
        Self(ip)
    }
}

impl From<IpAddr> for std::net::IpAddr {
    fn from(ip: IpAddr) -> Self {
        ip.0
    }
}

/// User agent parser result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentInfo {
    pub raw: String,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub device: Option<String>,
    pub is_mobile: bool,
    pub is_bot: bool,
}

/// Geographic location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
}

/// Money amount with currency (ISO 4217)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64, // Minor units (cents)
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Currency(pub String);

impl Currency {
    pub const USD: Self = Self(*b"USD");
    pub const EUR: Self = Self(*b"EUR");
    pub const GBP: Self = Self(*b"GBP");
    pub const JPY: Self = Self(*b"JPY");
    pub const CNY: Self = Self(*b"CNY");
    pub const INR: Self = Self(*b"INR");
    pub const BRL: Self = Self(*b"BRL");

    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into().to_uppercase())
    }

    pub fn minor_unit(&self) -> u8 {
        match self.0.as_str() {
            "JPY" | "KRW" | "VND" | "CLP" => 0,
            "BHD" | "JOD" | "KWD" | "OMR" | "TND" => 3,
            _ => 2,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self::new(0, currency)
    }

    pub fn major_units(&self) -> f64 {
        self.amount as f64 / 10_f64.powi(self.currency.minor_unit() as i32)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {}", self.major_units(), self.currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generation() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
        assert!(id1.to_string().starts_with("usr_"));
    }

    #[test]
    fn id_parsing() {
        let id = UserId::new();
        let parsed: UserId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn timestamp() {
        let ts = Timestamp::now();
        assert!(ts.as_millis() > 0);
    }

    #[test]
    fn pagination() {
        let p = Pagination::new(2, 25);
        assert_eq!(p.offset(), 25);
        assert_eq!(p.limit(), 25);
    }

    #[test]
    fn email_validation() {
        assert!(Email::new("test@example.com").is_ok());
        assert!(Email::new("invalid").is_err());
    }

    #[test]
    fn money() {
        let money = Money::new(1999, Currency::USD);
        assert_eq!(money.major_units(), 19.99);
    }
}