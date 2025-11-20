use config::{Config, File};
use entities::LoggingConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Deprecated: use logging.env_filter instead
    #[serde(default)]
    pub env_filter: Option<String>,
    pub listenner_addr: String,
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

        let mut settings: Settings = builder.build()?.try_deserialize()?;
        
        // Backward compatibility: if env_filter is provided, use it
        if let Some(env_filter) = &settings.env_filter {
            settings.logging.env_filter = env_filter.clone();
        }
        
        Ok(settings)
    }
}
