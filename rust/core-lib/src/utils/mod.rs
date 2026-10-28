//! Utilities
//!
//! Common utility functions for ID generation, time, string manipulation, etc.

use crate::domain::{Timestamp, Locale, MimeType, FileSize, ValidatedUrl, Email, PhoneNumber};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use ulid::Ulid;
use uuid::Uuid;

/// Generate a new ULID
pub fn generate_id() -> Ulid {
    Ulid::new()
}

/// Generate a new ULID as string
pub fn generate_ulid() -> String {
    Ulid::new().to_string()
}

/// Generate a UUID v7 (timestamp-based)
pub fn generate_uuid_v7() -> Uuid {
    Uuid::now_v7()
}

/// Current timestamp in milliseconds
pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Current timestamp in seconds
pub fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Current timestamp
pub fn now() -> Timestamp {
    Timestamp::now()
}

/// Parse datetime string to Timestamp
pub fn parse_datetime(s: &str) -> Result<Timestamp, chrono::ParseError> {
    let dt = DateTime::parse_from_rfc3339(s)?;
    Ok(Timestamp::from(dt.timestamp_millis()))
}

/// Format timestamp as RFC3339 string
pub fn format_datetime(ts: Timestamp) -> String {
    DateTime::<Utc>::from_timestamp_millis(ts.as_millis())
        .unwrap_or_default()
        .to_rfc3339()
}

/// Format timestamp as ISO 8601 date
pub fn format_date(ts: Timestamp) -> String {
    DateTime::<Utc>::from_timestamp_millis(ts.as_millis())
        .unwrap_or_default()
        .format("%Y-%m-%d")
        .to_string()
}

/// Format timestamp as ISO 8601 time
pub fn format_time(ts: Timestamp) -> String {
    DateTime::<Utc>::from_timestamp_millis(ts.as_millis())
        .unwrap_or_default()
        .format("%H:%M:%S%.3f")
        .to_string()
}

/// Calculate duration between two timestamps
pub fn duration_between(start: Timestamp, end: Timestamp) -> Duration {
    Duration::from_millis((end.as_millis() - start.as_millis()).abs() as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration {
    pub millis: i64,
}

impl Duration {
    pub const ZERO: Self = Self { millis: 0 };
    
    pub fn from_millis(millis: i64) -> Self {
        Self { millis }
    }
    
    pub fn from_secs(secs: i64) -> Self {
        Self { millis: secs * 1000 }
    }
    
    pub fn from_mins(mins: i64) -> Self {
        Self { millis: mins * 60 * 1000 }
    }
    
    pub fn from_hours(hours: i64) -> Self {
        Self { millis: hours * 60 * 60 * 1000 }
    }
    
    pub fn from_days(days: i64) -> Self {
        Self { millis: days * 24 * 60 * 60 * 1000 }
    }
    
    pub fn as_millis(&self) -> i64 {
        self.millis
    }
    
    pub fn as_secs(&self) -> i64 {
        self.millis / 1000
    }
    
    pub fn as_mins(&self) -> i64 {
        self.millis / (60 * 1000)
    }
    
    pub fn as_hours(&self) -> i64 {
        self.millis / (60 * 60 * 1000)
    }
    
    pub fn as_days(&self) -> i64 {
        self.millis / (24 * 60 * 60 * 1000)
    }
}

impl std::ops::Add for Duration {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self { millis: self.millis + other.millis }
    }
}

impl std::ops::Sub for Duration {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self { millis: self.millis - other.millis }
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let millis = self.millis.abs();
        let sign = if self.millis < 0 { "-" } else { "" };
        
        let days = millis / (24 * 60 * 60 * 1000);
        let hours = (millis % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000);
        let mins = (millis % (60 * 60 * 1000)) / (60 * 1000);
        let secs = (millis % (60 * 1000)) / 1000;
        let ms = millis % 1000;
        
        if days > 0 {
            write!(f, "{}d {}h {}m", sign, days, hours)
        } else if hours > 0 {
            write!(f, "{}h {}m", sign, hours, mins)
        } else if mins > 0 {
            write!(f, "{}m {}s", sign, mins, secs)
        } else if secs > 0 {
            write!(f, "{}s", sign, secs)
        } else {
            write!(f, "{}ms", sign, ms)
        }
    }
}

/// Slugify a string
pub fn slugify(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_was_separator = false;
    
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            prev_was_separator = false;
        } else if c.is_whitespace() || c == '-' || c == '_' {
            if !prev_was_separator && !result.is_empty() {
                result.push('-');
                prev_was_separator = true;
            }
        }
    }
    
    // Trim trailing separator
    while result.ends_with('-') {
        result.pop();
    }
    
    result
}

/// Truncate string to max length
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Sanitize string for safe display
pub fn sanitize(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect()
}

/// Mask sensitive string (e.g., for logging)
pub fn mask_string(s: &str, visible: usize) -> String {
    if s.len() <= visible * 2 {
        "*".repeat(s.len())
    } else {
        format!("{}***{}", &s[..visible], &s[s.len() - visible..])
    }
}

/// Generate random string
pub fn random_string(len: usize) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut result = String::with_capacity(len);
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        result.push(CHARS[rng.gen_range(0..CHARS.len())] as char);
    }
    result
}

/// Generate random alphanumeric string
pub fn random_alphanumeric(len: usize) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut result = String::with_capacity(len);
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        result.push(CHARS[rng.gen_range(0..CHARS.len())] as char);
    }
    result
}

/// Generate random numeric string
pub fn random_numeric(len: usize) -> String {
    let mut result = String::with_capacity(len);
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        result.push((b'0' + rng.gen_range(0..10)) as char);
    }
    result
}

/// Base64 encode
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Base64 decode
pub fn base64_decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(s)
}

/// URL-safe Base64 encode
pub fn base64_url_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// URL-safe Base64 decode
pub fn base64_url_decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(s)
}

/// Hash string using FNV-1a
pub fn hash_string(s: &str) -> u64 {
    let mut hasher = fnv::FnvHasher::default();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Hash bytes using FNV-1a
pub fn hash_bytes(data: &[u8]) -> u64 {
    let mut hasher = fnv::FnvHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Consistent hash for sharding
pub fn consistent_hash(key: &str, replicas: u32) -> u64 {
    let mut hasher = fnv::FnvHasher::default();
    format!("{}:{}", key, replicas).hash(&mut hasher);
    hasher.finish()
}

/// Parse query string into HashMap
pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for pair in query.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.insert(urlencoding::decode(key).unwrap().to_string(), urlencoding::decode(value).unwrap().to_string());
        }
    }
    params
}

/// Build query string from HashMap
pub fn build_query_string(params: &HashMap<String, String>) -> String {
    params.iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// Extract domain from URL
pub fn extract_domain(url: &str) -> Option<String> {
    ValidatedUrl::parse(url).ok().and_then(|u| u.host_str().map(String::from))
}

/// Extract subdomain from host
pub fn extract_subdomain(host: &str, root_domain: &str) -> Option<String> {
    if host == root_domain {
        return None;
    }
    
    if host.ends_with(&format!(".{}", root_domain)) {
        let subdomain = host.trim_end_matches(&format!(".{}", root_domain));
        if !subdomain.is_empty() && !subdomain.contains('.') {
            return Some(subdomain.to_string());
        }
    }
    
    None
}

/// Check if string is valid hex
pub fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Convert hex string to bytes
pub fn hex_decode(s: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(s)
}

/// Convert bytes to hex string
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

/// Deep merge two JSON values
pub fn deep_merge_json(a: &mut serde_json::Value, b: serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(k);
                } else {
                    deep_merge_json(a.entry(k).or_insert(serde_json::Value::Null), v);
                }
            }
        }
        (a, b) => *a = b,
    }
}

/// Flatten JSON object with dot notation
pub fn flatten_json(value: &serde_json::Value, prefix: &str, result: &mut HashMap<String, serde_json::Value>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_json(v, &new_prefix, result);
            }
        }
        _ => {
            result.insert(prefix.to_string(), value.clone());
        }
    }
}

/// Unflatten JSON object from dot notation
pub fn unflatten_json(flat: HashMap<String, serde_json::Value>) -> serde_json::Value {
    let mut root = serde_json::Map::new();
    
    for (key, value) in flat {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &mut root;
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                current.insert(part.to_string(), value);
            } else {
                current = current.entry(part.to_string()).or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .unwrap();
            }
        }
    }
    
    serde_json::Value::Object(root)
}

/// Calculate percentage
pub fn percentage(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// Calculate percentile from sorted values
pub fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    
    let index = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    
    if lower == upper {
        sorted[lower]
    } else {
        let weight = index - lower as f64;
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }
}

/// Moving average calculator
pub struct MovingAverage {
    values: Vec<f64>,
    window: usize,
    sum: f64,
}

impl MovingAverage {
    pub fn new(window: usize) -> Self {
        Self {
            values: Vec::with_capacity(window),
            window,
            sum: 0.0,
        }
    }
    
    pub fn add(&mut self, value: f64) -> f64 {
        self.values.push(value);
        self.sum += value;
        
        if self.values.len() > self.window {
            self.sum -= self.values.remove(0);
        }
        
        self.average()
    }
    
    pub fn average(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.sum / self.values.len() as f64
        }
    }
    
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
    }
}

/// Rate limiter token bucket
pub struct TokenBucket {
    capacity: u64,
    tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: std::time::Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate,
            last_refill: std::time::Instant::now(),
        }
    }
    
    pub fn try_consume(&mut self, tokens: u64) -> bool {
        self.refill();
        
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            true
        } else {
            false
        }
    }
    
    pub fn available(&mut self) -> u64 {
        self.refill();
        self.tokens.floor() as u64
    }
    
    fn refill(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity as f64);
        self.last_refill = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generation() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn slugify_test() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("  Hello   World  "), "hello-world");
        assert_eq!(slugify("Test_Case"), "test-case");
        assert_eq!(slugify("Multiple---Separators"), "multiple-separators");
    }

    #[test]
    fn truncate_test() {
        assert_eq!(truncate("Hello", 10), "Hello");
        assert_eq!(truncate("Hello World", 8), "Hello...");
    }

    #[test]
    fn mask_string_test() {
        assert_eq!(mask_string("secret", 2), "se***et");
        assert_eq!(mask_string("ab", 2), "**");
    }

    #[test]
    fn base64_test() {
        let data = b"Hello, World!";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(data.to_vec(), decoded);
    }

    #[test]
    fn duration_test() {
        let d = Duration::from_secs(65);
        assert_eq!(d.as_millis(), 65000);
        assert_eq!(d.as_secs(), 65);
    }

    #[test]
    fn percentage_test() {
        assert_eq!(percentage(50, 100), 50.0);
        assert_eq!(percentage(1, 3), 33.33333333333333);
        assert_eq!(percentage(0, 100), 0.0);
    }

    #[test]
    fn percentile_test() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&data, 50.0), 3.0);
        assert_eq!(percentile(&data, 0.0), 1.0);
        assert_eq!(percentile(&data, 100.0), 5.0);
    }

    #[test]
    fn moving_average_test() {
        let mut ma = MovingAverage::new(3);
        assert_eq!(ma.add(1.0), 1.0);
        assert_eq!(ma.add(2.0), 1.5);
        assert_eq!(ma.add(3.0), 2.0);
        assert_eq!(ma.add(4.0), 3.0); // (2+3+4)/3
    }

    #[test]
    fn token_bucket_test() {
        let mut bucket = TokenBucket::new(10, 1.0); // 10 capacity, 1 token/sec
        assert!(bucket.try_consume(5));
        assert_eq!(bucket.available(), 5);
        
        // Wait a bit
        std::thread::sleep(std::time::Duration::from_millis(1100));
        assert_eq!(bucket.available(), 6);
    }
}