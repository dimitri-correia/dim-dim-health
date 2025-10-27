use crate::axummain::{env_loader::Settings, router, state};

pub async fn axum_main() {
    let settings = Settings::load_config().expect("Failed to load configuration");

    let app_state = state::AppState::new(settings)
        .await
        .expect("Failed to create AppState");

    let app = router::get_main_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
