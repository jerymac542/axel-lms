//! Rate Limiting Middleware

use apexos_core::{
    config::{RateLimitRule, RateLimitStorage, KeyGenerator},
    domain::CorrelationId,
    errors::{ErrorCode, Result, ApexosError},
    http::{AppState, AppState},
};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn, instrument};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

// ============================================================================
// Rate Limiting Middleware
// ============================================================================

/// Rate limiting middleware with configurable rules
#[instrument(skip(state, request, next))]
pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, crate::errors::ApexosError> {
    // Extract identifier for rate limiting
    let identifier = extract_rate_limit_identifier(&request);
    
    // Get rate limit rule for this endpoint
    let rule = get_rate_limit_rule(&request, &state.settings.security.rate_limiting);
    
    // Check rate limit
    check_rate_limit(&state, &identifier, &rule).await?;
    
    // Add rate limit headers to response
    let mut response = next.run(request).await;
    
    // Add rate limit headers
    add_rate_limit_headers(&mut response, &rule);
    
    Ok(response)
}

/// Extract identifier for rate limiting
fn extract_rate_limit_identifier(request: &axum::http::Request<axum::body::Body>) -> String {
    // Try API key first
    if let Some(api_key) = request.headers().get("x-api-key").and_then(|v| v.to_str().ok()) {
        return format!("apikey:{}", api_key);
    }
    
    // Try user ID from auth (would be set by auth middleware)
    // For now, fall back to IP
    
    // Try X-Forwarded-For header
    if let Some(forwarded) = request.headers().get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        return format!("ip:{}", forwarded.split(',').next().unwrap_or("unknown").trim());
    }
    
    // Try X-Real-IP header
    if let Some(real_ip) = request.headers().get("x-real-ip").and_then(|v| v.to_str().ok()) {
        return format!("ip:{}", real_ip);
    }
    
    // Fallback to IP from connection info (would need extension)
    "ip:unknown".to_string()
}

/// Get rate limit rule for request
fn get_rate_limit_rule(
    request: &axum::http::Request<axum::body::Body>,
    config: &crate::config::RateLimitingConfig,
) -> crate::config::RateLimitRule {
    let path = request.uri().path();
    
    // Check for specific path rules
    for (pattern, rule) in &config.rules {
        if path_matches_pattern(request.uri().path(), pattern) {
            return rule.clone();
        }
    }
    
    // Return default rule
    config.default.clone()
}

/// Simple path pattern matching
fn path_matches_pattern(path: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    if pattern.ends_with("*") {
        let prefix = &pattern[..pattern.len()-1];
        return path.starts_with(prefix);
    }
    
    path == pattern
}

/// Check rate limit for identifier
async fn check_rate_limit(
    state: &crate::http::AppState,
    identifier: &str,
    rule: &crate::config::RateLimitRule,
) -> Result<(), crate::errors::ApexosError> {
    let key = format!("ratelimit:{}", identifier);
    
    // Use Redis for distributed rate limiting
    let mut conn = state.cache.get_connection().await?;
    
    let current: u64 = redis::cmd("INCR")
        .arg(&key)
        .query_async(&mut conn)
        .await
        .map_err(|e| crate::errors::ApexosError::new(crate::errors::ErrorCode::InternalError, e.to_string()))?;
    
    if current == 1 {
        // Set expiry on first request
        let _: () = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(rule.window.as_secs() as i64)
            .query_async(&mut conn)
            .await
            .map_err(|e| crate::errors::ApexosError::new(crate::errors::ErrorCode::InternalError, e.to_string()))?;
    }
    
    if current > rule.max_requests as u64 {
        // Set retry-after header
        let retry_after = rule.window.as_secs();
        
        return Err(crate::errors::ApexosError::new(
            crate::errors::ErrorCode::RateLimited,
            "Rate limit exceeded"
        ).with_field("retry_after", retry_after.to_string()));
    }
    
    Ok(())
}

/// Add rate limit headers to response
fn add_rate_limit_headers(response: &mut axum::response::Response, rule: &crate::config::RateLimitRule) {
    let headers = response.headers_mut();
    
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_str(&rule.max_requests.to_string()).unwrap(),
    );
    
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str("0").unwrap(), // Would calculate actual remaining
    );
    
    headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from_str(&(chrono::Utc::now() + rule.window).timestamp().to_string()).unwrap(),
    );
}

/// Rate limiter for specific endpoints
pub struct EndpointRateLimiter {
    rules: HashMap<String, crate::config::RateLimitRule>,
    default_rule: crate::config::RateLimitRule,
}

impl EndpointRateLimiter {
    pub fn new(config: &crate::config::RateLimitingConfig) -> Self {
        Self {
            rules: config.rules.clone(),
            default_rule: config.default.clone(),
        }
    }
    
    pub fn check(&self, path: &str, identifier: &str) -> Result<(), crate::errors::ApexosError> {
        let rule = self.rules.get(path).unwrap_or(&self.default_rule);
        
        // In-memory rate limiting (for single instance)
        // In production, use Redis
        // This is a simplified version
        
        Ok(())
    }
}

// In-memory rate limiter for development/testing
pub struct MemoryRateLimiter {
    buckets: Mutex<HashMap<String, RateLimitBucket>>,
    default_rule: crate::config::RateLimitRule,
}

#[derive(Debug, Clone)]
struct RateLimitBucket {
    count: u32,
    window_start: std::time::Instant,
    blocked_until: Option<Instant>,
}

impl MemoryRateLimiter {
    pub fn new(default_rule: crate::config::RateLimitRule) -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            default_rule,
        }
    }
    
    pub fn check(&self, key: &str, rule: Option<&crate::config::RateLimitRule>) -> Result<(), crate::errors::ApexosError> {
        let rule = rule.unwrap_or(&self.default_rule);
        let mut buckets = self.buckets.lock().unwrap();
        
        let now = Instant::now();
        let bucket = buckets.entry(key.to_string()).or_insert(RateLimitBucket {
            count: 0,
            window_start: now,
            blocked_until: None,
        });
        
        // Check if blocked
        if let Some(blocked_until) = bucket.blocked_until {
            if now < blocked_until {
                return Err(crate::errors::ApexosError::new(
                    crate::errors::ErrorCode::RateLimited,
                    "Rate limited"
                ));
            } else {
                bucket.blocked_until = None;
            }
        }
        
        // Reset window if expired
        if now.duration_since(bucket.window_start) > rule.window {
            bucket.count = 0;
            bucket.window_start = now;
        }
        
        bucket.count += 1;
        
        if bucket.count > rule.max_requests {
            bucket.blocked_until = Some(now + rule.block_duration.unwrap_or(Duration::from_secs(900)));
            return Err(crate::errors::ApexosError::new(
                crate::errors::ErrorCode::RateLimited,
                "Rate limit exceeded"
            ).with_field("retry_after", rule.window.as_secs().to_string()));
        }
        
        Ok(())
    }
}

use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Instant;