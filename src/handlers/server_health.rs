use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::axummain::state::AppState;

pub async fn server_health_check(State(state): State<AppState>) -> Json<Value> {
    match sqlx::query("SELECT 1 AS value").fetch_one(&state.db).await {
        Ok(_) => Json(json!({
            "status": "ok"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Json(json!({
                "status": "error",
            }))
        }
    }
}
