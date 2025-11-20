use entities::{LogFormat, LoggingConfig};
use tracing::{error, info, warn};

fn main() {
    println!("=== JSON Logging Format Demo ===\n");
    
    let config = LoggingConfig::from_env_filter("info")
        .with_format(LogFormat::Json)
        .with_file(true)
        .with_line_number(true);
    
    config.init().expect("Failed to initialize logging");
    
    info!("Application starting");
    info!(version = "1.0.0", env = "production", "Configuration loaded");
    warn!(retry_count = 3, "Retrying operation");
    error!(error_code = "DB_001", "Database connection failed");
    
    let user_id = "user-456";
    let _span = tracing::info_span!("api_request", user_id = %user_id, endpoint = "/api/login").entered();
    info!("Processing API request");
    info!(action = "login", status = "success", latency_ms = 42, "Request completed");
    
    println!("\n✓ JSON logging demo completed - note the structured output above!");
}
