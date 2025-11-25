use entities::env_loader::Settings;
use tokio::signal;
use tracing::{error, info, warn};

use crate::axummain::{router, state};

pub async fn axum_main() {
    let settings = match Settings::load_config() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = settings.init_telemetry("dimdim-health-api") {
        eprintln!("Failed to initialize telemetry: {}", e);
        std::process::exit(1);
    }

    info!(
        listen_addr = %settings.listenner_addr,
        "Starting DimDim Health API server"
    );

    let app_state = match state::AppState::create_from_settings(&settings).await {
        Ok(s) => {
            info!("Application state initialized successfully");
            s
        }
        Err(e) => {
            error!(error = %e, "Failed to create application state");
            std::process::exit(1);
        }
    };

    let app = router::get_main_router(app_state);

    let listener = match tokio::net::TcpListener::bind(&settings.listenner_addr).await {
        Ok(l) => l,
        Err(e) => {
            error!(
                error = %e,
                addr = %settings.listenner_addr,
                "Failed to bind to address"
            );
            std::process::exit(1);
        }
    };

    info!(
        addr = %settings.listenner_addr,
        "Server listening and ready to accept connections"
    );

    // Graceful shutdown with signal handling
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!(error = %e, "Server error");
    }

    info!("Server shutting down gracefully...");

    // Shutdown telemetry gracefully
    settings.shutdown_telemetry();
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
            warn!("Received Ctrl+C signal, initiating graceful shutdown...");
        },
        _ = terminate => {
            warn!("Received SIGTERM signal, initiating graceful shutdown...");
        },
    }
}
