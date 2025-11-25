use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::{error, info, instrument};

use crate::axummain::state::AppState;

/// Response structure for health check endpoint
#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Health check endpoint that verifies database connectivity
#[instrument(name = "health_check", skip(state))]
pub async fn server_health_check(State(state): State<AppState>) -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION").to_string();

    match state.db.ping().await {
        Ok(_) => {
            info!(
                status = "healthy",
                database = "connected",
                "Health check passed"
            );
            (
                StatusCode::OK,
                Json(HealthCheckResponse {
                    status: "healthy".to_string(),
                    version,
                    database: Some("connected".to_string()),
                    error: None,
                }),
            )
                .into_response()
        }
        Err(e) => {
            error!(
                status = "unhealthy",
                database = "disconnected",
                error = %e,
                "Health check failed - database unreachable"
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HealthCheckResponse {
                    status: "unhealthy".to_string(),
                    version,
                    database: Some("disconnected".to_string()),
                    error: Some("Database connection failed".to_string()),
                }),
            )
                .into_response()
        }
    }
}
