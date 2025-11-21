use log::info;

use crate::axummain::{env_loader::Settings, router, state, telemetry};

pub async fn axum_main() {
    let settings = Settings::load_config().expect("Failed to load configuration");

    // Initialize telemetry with OpenObserve if configured
    telemetry::init_telemetry(
        "dimdim-health-api",
        settings.openobserve_endpoint.as_deref(),
        &settings.env_filter,
    )
    .expect("Failed to initialize telemetry");

    info!("Starting Axum server...");

    let app_state = state::AppState::create_from_settings(&settings)
        .await
        .expect("Failed to create AppState");

    let app = router::get_main_router(app_state);

    let listener = tokio::net::TcpListener::bind(&settings.listenner_addr)
        .await
        .unwrap();

    info!("Server listening on {}", &settings.listenner_addr);

    // Setup graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    // Run server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();

    info!("Server shutting down...");
    
    // Shutdown telemetry gracefully
    telemetry::shutdown_telemetry();
}
