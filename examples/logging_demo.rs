use entities::{LogFormat, LoggingConfig};
use tracing::{error, info, warn};

fn main() {
    println!("=== Logging System Demo ===\n");
    
    // Demo: Pretty format with different log levels
    println!("Pretty Format Demo:");
    println!("-------------------");
    
    let config = LoggingConfig::from_env_filter("info")
        .with_format(LogFormat::Pretty)
        .with_file(true)
        .with_line_number(true);
    
    config.init().expect("Failed to initialize logging");
    
    info!("Application starting");
    info!(version = "1.0.0", env = "development", "Configuration loaded");
    warn!(retry_count = 3, "Retrying operation");
    error!(error_code = "DB_001", "Database connection failed");
    
    // Simulate a span
    let user_id = "user-123";
    let _span = tracing::info_span!("user_request", user_id = %user_id).entered();
    info!("Processing user request");
    info!(action = "login", status = "success", "User action completed");
    
    println!("\n✓ Logging demo completed successfully!");
    println!("\nTo see JSON format, run with: cargo run --example logging_demo_json");
    println!("To see compact format, run with: cargo run --example logging_demo_compact");
}
