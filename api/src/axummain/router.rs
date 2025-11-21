use axum::http::{HeaderValue, Method, header};
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

use crate::axummain::state::AppState;
use crate::handlers::auth::{
    current_user, forgot_password, login, logout, refresh_token, register, register_guest,
    reset_password, verify_email,
};
use crate::handlers::food_item::{
    create_food_item, delete_food_item, get_food_item_by_id, get_food_items, update_food_item,
};
use crate::handlers::meal::{
    add_meal_item, create_meal, delete_meal, delete_meal_item, get_meal_by_id, get_meal_items,
    get_meals, update_meal, update_meal_item,
};
use crate::handlers::server_health::server_health_check;
use crate::handlers::user_weight::{
    create_user_weight, delete_user_weight, get_user_weights, update_user_weight,
};

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
        // User weight routes
        .route("/api/user/weights", post(create_user_weight))
        .route("/api/user/weights", get(get_user_weights))
        .route("/api/user/weights/{id}", put(update_user_weight))
        .route("/api/user/weights/{id}", delete(delete_user_weight))
        // Food item routes
        .route("/api/food-items", post(create_food_item))
        .route("/api/food-items", get(get_food_items))
        .route("/api/food-items/{id}", get(get_food_item_by_id))
        .route("/api/food-items/{id}", put(update_food_item))
        .route("/api/food-items/{id}", delete(delete_food_item))
        // Meal routes
        .route("/api/meals", post(create_meal))
        .route("/api/meals", get(get_meals))
        .route("/api/meals/{id}", get(get_meal_by_id))
        .route("/api/meals/{id}", put(update_meal))
        .route("/api/meals/{id}", delete(delete_meal))
        // Meal item routes
        .route("/api/meals/{meal_id}/items", post(add_meal_item))
        .route("/api/meals/{meal_id}/items", get(get_meal_items))
        .route(
            "/api/meals/{meal_id}/items/{item_id}",
            put(update_meal_item),
        )
        .route(
            "/api/meals/{meal_id}/items/{item_id}",
            delete(delete_meal_item),
        )
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
