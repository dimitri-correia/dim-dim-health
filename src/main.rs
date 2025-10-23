use axum::{Router, extract::State, response::Json, routing::get};
use serde_json::{Value, json};

use crate::state::AppState;
mod state;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Database setup
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _app_state = state::AppState::new(&database_url)
        .await
        .expect("Failed to create AppState");

    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(_app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check(State(state): State<AppState>) -> Json<Value> {
    match sqlx::query!("SELECT 1 AS value").fetch_one(&state.db).await {
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
