use axum::routing::post;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::axummain::state::AppState;
use crate::handlers::auth::{
    current_user, forgot_password, login, logout, refresh_token, register, reset_password,
    verify_email,
};
use crate::handlers::food_item::{
    create_food_item, delete_food_item, get_food_item, get_food_items, update_food_item,
};
use crate::handlers::meal::{create_meal, delete_meal, get_meal, get_user_meals, update_meal};
use crate::handlers::meal_item::{
    create_meal_item, delete_meal_item, get_meal_items, update_meal_item,
};
use crate::handlers::server_health::server_health_check;
use crate::handlers::user_weight::{
    create_user_weight, delete_user_weight, get_latest_user_weight, get_user_weights,
    update_user_weight,
};

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
        .route("/api/auth/logout", post(logout))
        // User weight routes
        .route("/api/user/weights", post(create_user_weight))
        .route("/api/user/weights", get(get_user_weights))
        .route("/api/user/weights/latest", get(get_latest_user_weight))
        .route("/api/user/weights/:id", axum::routing::put(update_user_weight))
        .route("/api/user/weights/:id", axum::routing::delete(delete_user_weight))
        // Meal routes
        .route("/api/meals", post(create_meal))
        .route("/api/meals", get(get_user_meals))
        .route("/api/meals/:id", get(get_meal))
        .route("/api/meals/:id", axum::routing::put(update_meal))
        .route("/api/meals/:id", axum::routing::delete(delete_meal))
        // Meal item routes
        .route("/api/meal-items", post(create_meal_item))
        .route("/api/meals/:meal_id/items", get(get_meal_items))
        .route("/api/meal-items/:id", axum::routing::put(update_meal_item))
        .route("/api/meal-items/:id", axum::routing::delete(delete_meal_item))
        // Food item routes
        .route("/api/food-items", post(create_food_item))
        .route("/api/food-items", get(get_food_items))
        .route("/api/food-items/:id", get(get_food_item))
        .route("/api/food-items/:id", axum::routing::put(update_food_item))
        .route("/api/food-items/:id", axum::routing::delete(delete_food_item))
        // Set application state
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
