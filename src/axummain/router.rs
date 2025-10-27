use axum::{Router, routing::get};

use crate::axummain::state::AppState;
use crate::handlers::server_health::server_health_check;

pub fn get_main_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(server_health_check))
        .with_state(app_state)
}
