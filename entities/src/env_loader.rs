use anyhow::Result;
use config::{Config, File};
use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_otlp::{HttpExporterBuilder, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::{
    Resource,
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

static TRACER_PROVIDER: OnceLock<SdkTracerProvider> = OnceLock::new();

const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub env: String,

    pub database_url: String,
    pub redis_url: String,

    pub frontend_url: String,
    pub listenner_addr: String,

    pub env_filter: String,

    pub jwt_secret: String,

    pub number_workers: usize,

    pub gmail_email: String,
    pub gmail_password: String,

    pub openobserve_endpoint: String,
    pub openobserve_user: String,
    pub openobserve_password: String,
}

impl Settings {
    pub fn load_config() -> Result<Self, config::ConfigError> {
        // Determine the environment (default to "dev" if not set)
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into());

        let builder = Config::builder()
            // Load common first
            .add_source(File::with_name("config/common.toml").required(true))
            // Override with env-specific file
            .add_source(File::with_name(&format!("config/{}.toml", env)).required(false))
            .set_default("env", env)?;

        builder.build()?.try_deserialize()
    }

    pub fn init_telemetry(&self, service_name: &str) -> Result<()> {
        let filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&self.env_filter));

        // Create Basic Auth header for OpenObserve
        use base64::Engine;
        let credentials = format!("{}:{}", self.openobserve_user, self.openobserve_password);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
        let auth_header = format!("Basic {}", encoded);

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), auth_header);

        let exporter = HttpExporterBuilder::default()
            .with_endpoint(&self.openobserve_endpoint)
            .with_timeout(std::time::Duration::from_secs(3))
            .with_http_client(reqwest::Client::new())
            .with_headers(headers)
            .build_span_exporter()?;

        let resource = Resource::builder_empty()
            .with_attributes([
                KeyValue::new("service.name", service_name.to_string()),
                KeyValue::new("service.version", SERVICE_VERSION.to_string()),
                KeyValue::new("deployment.environment", self.env.clone()),
                KeyValue::new("service.instance.id", uuid::Uuid::new_v4().to_string()),
            ])
            .build();

        let provider = SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(resource)
            .with_sampler(Sampler::AlwaysOn)
            .with_id_generator(RandomIdGenerator::default())
            .with_max_events_per_span(64)
            .with_max_attributes_per_span(32)
            .build();

        // Set as global provider
        global::set_tracer_provider(provider.clone());

        // Store provider for shutdown
        let _ = TRACER_PROVIDER.set(provider.clone());

        // Create telemetry layer
        let tracer_name = service_name.to_string();
        let telemetry = tracing_opentelemetry::layer().with_tracer(provider.tracer(tracer_name));

        // Initialize tracing subscriber with both console and OpenTelemetry layers
        tracing_subscriber::registry()
            .with(filter.clone())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_filter(filter),
            )
            .with(telemetry)
            .init();

        tracing::info!(
            service.name = service_name,
            service.version = SERVICE_VERSION,
            environment = %self.env,
            otlp.endpoint = %self.openobserve_endpoint,
            "Telemetry initialized with OpenTelemetry"
        );

        Ok(())
    }

    pub fn shutdown_telemetry(&self) {
        tracing::info!("Shutting down telemetry...");
        if let Some(provider) = TRACER_PROVIDER.get() {
            if let Err(e) = provider.shutdown() {
                tracing::error!(error = %e, "Failed to shutdown telemetry provider");
            } else {
                tracing::info!("Telemetry shutdown complete");
            }
        }
    }
}
