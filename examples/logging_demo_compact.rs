use entities::{LogFormat, LoggingConfig};
use tracing::{error, info, warn};

fn main() {
    println!("=== Compact Logging Format Demo ===\n");
    
    let config = LoggingConfig::from_env_filter("info")
        .with_format(LogFormat::Compact)
        .with_file(false)
        .with_line_number(false);
    
    config.init().expect("Failed to initialize logging");
    
    info!("Application starting");
    info!(version = "1.0.0", "Configuration loaded");
    warn!("Retrying operation");
    error!("Database connection failed");
    
    let request_id = "req-789";
    let _span = tracing::info_span!("worker_task", request_id = %request_id).entered();
    info!("Processing background job");
    info!("Job completed");
    
    println!("\n✓ Compact logging demo completed - note the condensed output above!");
}
