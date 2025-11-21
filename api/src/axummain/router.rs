use axum::routing::post;
use axum::{Router, routing::get};
use axum::http::{header, HeaderValue, Method};
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::axummain::state::AppState;
use crate::handlers::auth::{
    current_user, forgot_password, login, logout, refresh_token, register, register_guest,
    reset_password, verify_email,
};
use crate::handlers::server_health::server_health_check;
use crate::handlers::user_group::join_public_group;

pub fn get_main_router(app_state: AppState) -> Router {
    // Configure CORS - adjust allowed origins for production
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:8081".parse::<HeaderValue>().unwrap(), // Flutter web default port
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        // Health check route
        .route("/health", get(server_health_check))
        // Auth routes
        .route("/api/users", post(register))
        .route("/api/users/guest", post(register_guest))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .route("/api/auth/verify-email", get(verify_email))
        .route("/api/auth/forgot-password", post(forgot_password))
        .route("/api/auth/reset-password", post(reset_password))
        .route("/api/auth/refresh-token", post(refresh_token))
        .route("/api/auth/logout", post(logout))
        // User group routes
        .route("/api/user-groups/join-public", post(join_public_group))
        // Set application state
        .with_state(app_state)
        // Security headers
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        // Apply CORS
        .layer(cors)
        // Apply tracing
        .layer(TraceLayer::new_for_http())
}
