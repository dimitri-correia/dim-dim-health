use axum::routing::post;
use axum::{Router, routing::get};

use crate::axummain::state::AppState;
use crate::handlers::auth::{current_user, login, register};
use crate::handlers::server_health::server_health_check;

pub fn get_main_router(app_state: AppState) -> Router {
    Router::new()
        // Health check route
        .route("/health", get(server_health_check))
        // Auth routes
        .route("/api/users", post(register))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        // Set application state
        .with_state(app_state)
}
