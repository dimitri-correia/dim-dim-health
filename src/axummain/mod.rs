use axum::{Router, routing::get};

use crate::handlers::server_health::server_health_check;
use crate::state;

pub async fn axum_main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Database setup
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _app_state = state::AppState::new(&database_url)
        .await
        .expect("Failed to create AppState");

    let app = Router::new()
        .route("/health", get(server_health_check))
        .with_state(_app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
