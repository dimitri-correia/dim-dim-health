use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,

    pub frontend_url: String,

    pub env_filter: String,

    pub number_workers: usize,

    pub gmail_email: String,
    pub gmail_password: String,

    pub openobserve_endpoint: Option<String>,
}

impl Settings {
    pub fn load_config() -> Result<Self, config::ConfigError> {
        // Determine the environment (default to "dev" if not set)
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into());

        let builder = Config::builder()
            // Load common first
            .add_source(File::with_name("config/common.toml").required(true))
            // Override with env-specific file
            .add_source(File::with_name(&format!("config/{}.toml", env)).required(false));

        builder.build()?.try_deserialize()
    }
}
