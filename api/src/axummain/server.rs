use log::info;

use crate::axummain::{env_loader::Settings, router, state};

pub async fn axum_main() {
    let settings = Settings::load_config().expect("Failed to load configuration");

    tracing_subscriber::fmt()
        .with_env_filter(&settings.env_filter)
        .init();

    info!("Starting Axum server...");

    let app_state = state::AppState::create_from_settings(&settings)
        .await
        .expect("Failed to create AppState");

    let app = router::get_main_router(app_state);

    let listener = tokio::net::TcpListener::bind(&settings.listenner_addr)
        .await
        .unwrap();

    info!("Server listening on {}", &settings.listenner_addr);

    axum::serve(listener, app).await.unwrap();
}
