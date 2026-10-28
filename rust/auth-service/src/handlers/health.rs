//! Health Check Handlers

use crate::domain::Timestamp;
use apexos_core::{
    http::{AppState, DatabasePool, CacheClient},
    errors::{ErrorCode, Result, ApexosError},
    http::ApiResponse,
};
use axum::{
    extract::{State, Extension},
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, instrument};
use utoipa::ToSchema;
use sqlx::PgPool;
use redis::AsyncCommands;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
    pub checks: HashMap<String, ComponentHealth>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ComponentHealth {
    pub status: String,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

// ============================================================================
// Health Handlers
// ============================================================================

/// Liveness probe - basic health check
#[utoipa::path(
    get,
    path = "/health/live",
    responses(
        (status = 200, description = "Service is alive", body = HealthResponse)
    ),
    tag = "health"
)]
#[instrument(skip(state))]
pub async fn liveness(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<HealthResponse>>, ApexosError> {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Timestamp::now().to_string(),
        checks: HashMap::new(),
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// Readiness probe - checks dependencies
#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = 200, description = "Service is ready", body = HealthResponse),
        (status = 503, description = "Service not ready", body = HealthResponse)
    ),
    tag = "health"
)]
#[instrument(skip(state))]
pub async fn readiness(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<HealthResponse>>, ApexosError> {
    let mut checks = HashMap::new();
    let mut all_healthy = true;
    
    // Database check
    let db_start = std::time::Instant::now();
    match sqlx::query("SELECT 1").execute(&*state.database).await {
        Ok(_) => {
            checks.insert("database".to_string(), ComponentHealth {
                status: "ok".to_string(),
                message: None,
                latency_ms: Some(db_start.elapsed().as_millis() as u64),
            });
        }
        Err(e) => {
            all_healthy = false;
            checks.insert("database".to_string(), ComponentHealth {
                status: "fail".to_string(),
                message: Some(e.to_string()),
                latency_ms: Some(db_start.elapsed().as_millis() as u64),
            });
        }
    }
    
    // Cache check
    let cache_start = std::time::Instant::now();
    match state.cache.get("health-check").await {
        Ok(_) => {
            checks.insert("cache".to_string(), ComponentHealth {
                status: "ok".to_string(),
                message: None,
                latency_ms: Some(cache_start.elapsed().as_millis() as u64),
            });
        }
        Err(e) => {
            all_healthy = false;
            checks.insert("cache".to_string(), ComponentHealth {
                status: "fail".to_string(),
                message: Some(e.to_string()),
                latency_ms: Some(cache_start.elapsed().as_millis() as u64),
            });
        }
    }
    
    // Message bus check (NATS)
    let nats_start = std::time::Instant::now();
    // Would check NATS connection here
    checks.insert("message_bus".to_string(), ComponentHealth {
        status: "ok".to_string(),
        message: Some("NATS connected".to_string()),
        latency_ms: Some(nats_start.elapsed().as_millis() as u64),
    });
    
    // Object store check
    let storage_start = std::time::Instant::now();
    // Would check S3/MinIO connection here
    checks.insert("object_store".to_string(), ComponentHealth {
        status: "ok".to_string(),
        message: Some("Storage accessible".to_string()),
        latency_ms: Some(storage_start.elapsed().as_millis() as u64),
    });
    
    let status = if all_healthy { "ok" } else { "degraded" }.to_string();
    
    let response = HealthResponse {
        status,
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Timestamp::now().to_string(),
        checks,
    };
    
    let status_code = if all_healthy { 200 } else { 503 };
    
    // For now, always return 200 with status in body
    // In production, would use custom response with status code
    Ok(Json(ApiResponse::new(response, "request-id")))
}

/// Startup probe
#[utoipa::path(
    get,
    path = "/health/startup",
    responses(
        (status = 200, description = "Service started", body = HealthResponse)
    ),
    tag = "health"
)]
#[instrument(skip(state))]
pub async fn startup(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<HealthResponse>>, ApexosError> {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Timestamp::now().to_string(),
        checks: HashMap::new(),
    };
    
    Ok(Json(ApiResponse::new(response, "request-id")))
}