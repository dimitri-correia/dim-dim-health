use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::axummain::state::AppState;

pub async fn server_health_check(State(state): State<AppState>) -> Json<Value> {
    match &state.db.ping().await {
        Ok(_) => Json(json!({"status": "ok"})),
        Err(_) => Json(json!({"status": "error"})),
    }
}
