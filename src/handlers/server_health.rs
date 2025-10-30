use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::axummain::state::AppState;

pub async fn server_health_check(State(state): State<AppState>) -> impl IntoResponse {
    match &state.db.ping().await {
        Ok(_) => (StatusCode::OK, Json(json!({"status": "ok"}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error"})),
        )
            .into_response(),
    }
}
