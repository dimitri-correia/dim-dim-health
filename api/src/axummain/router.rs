use axum::routing::post;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::axummain::state::AppState;
use crate::handlers::auth::{
    current_user, forgot_password, login, refresh_token, register, reset_password, verify_email,
};
use crate::handlers::server_health::server_health_check;

pub fn get_main_router(app_state: AppState) -> Router {
    Router::new()
        // Health check route
        .route("/health", get(server_health_check))
        // Auth routes
        .route("/api/users", post(register))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .route("/api/auth/verify-email", get(verify_email))
        .route("/api/auth/forgot-password", post(forgot_password))
        .route("/api/auth/reset-password", post(reset_password))
        .route("/api/auth/refresh-token", post(refresh_token))
        // Set application state
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
