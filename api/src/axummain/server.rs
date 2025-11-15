use log::info;
use tokio::signal;

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

    // Graceful shutdown with signal handling
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("Server shutdown complete");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal, initiating graceful shutdown...");
        },
        _ = terminate => {
            info!("Received SIGTERM signal, initiating graceful shutdown...");
        },
    }
}
