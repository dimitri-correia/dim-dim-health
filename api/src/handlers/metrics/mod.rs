use axum::{http::StatusCode, response::IntoResponse};
use log::error;

use crate::metrics;

/// Handler for the /metrics endpoint
/// Returns metrics in Prometheus text format
pub async fn metrics_handler() -> impl IntoResponse {
    match metrics::collect_metrics() {
        Ok(metrics_data) => (StatusCode::OK, metrics_data).into_response(),
        Err(e) => {
            error!("Failed to collect metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to collect metrics: {}", e),
            )
                .into_response()
        }
    }
}
