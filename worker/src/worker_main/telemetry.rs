use anyhow::Result;
use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use std::sync::OnceLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

/// Global tracer provider for shutdown
static TRACER_PROVIDER: OnceLock<SdkTracerProvider> = OnceLock::new();

/// Service version from Cargo.toml
const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize OpenTelemetry tracing with OpenObserve backend
///
/// This function sets up the OpenTelemetry tracing pipeline to export traces
/// to OpenObserve via OTLP (OpenTelemetry Protocol) over HTTP.
///
/// # Arguments
/// * `service_name` - The name of the service (e.g., "dimdim-health-worker")
/// * `otlp_endpoint` - The OpenObserve OTLP endpoint (e.g., "http://localhost:5080/api/default")
/// * `env_filter` - The environment filter for log levels
///
/// # Returns
/// * `Result<()>` - Ok if initialization succeeds, Err otherwise
pub fn init_telemetry(
    service_name: &str,
    otlp_endpoint: Option<&str>,
    env_filter: &str,
) -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Get environment from APP_ENV variable
    let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    // If no OTLP endpoint is provided, use standard tracing without OpenTelemetry
    if otlp_endpoint.is_none() || otlp_endpoint == Some("") {
        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();

        tracing::info!(
            service.name = service_name,
            service.version = SERVICE_VERSION,
            environment = %environment,
            "Telemetry initialized (console only)"
        );
        return Ok(());
    }

    let endpoint = otlp_endpoint.unwrap();

    // Create OpenTelemetry exporter
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .with_timeout(std::time::Duration::from_secs(3))
        .build()?;

    // Build resource attributes for better observability
    let resource = Resource::builder_empty()
        .with_attributes([
            KeyValue::new("service.name", service_name.to_string()),
            KeyValue::new("service.version", SERVICE_VERSION.to_string()),
            KeyValue::new("deployment.environment", environment.clone()),
            KeyValue::new("service.instance.id", uuid::Uuid::new_v4().to_string()),
        ])
        .build();

    // Create tracer provider
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(32)
        .with_resource(resource)
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
        environment = %environment,
        otlp.endpoint = endpoint,
        "Telemetry initialized with OpenTelemetry"
    );

    Ok(())
}

/// Shutdown OpenTelemetry gracefully
///
/// This should be called before the application exits to ensure all traces are flushed
pub fn shutdown_telemetry() {
    tracing::info!("Shutting down telemetry...");
    if let Some(provider) = TRACER_PROVIDER.get() {
        if let Err(e) = provider.shutdown() {
            tracing::error!(error = %e, "Failed to shutdown telemetry provider");
        } else {
            tracing::info!("Telemetry shutdown complete");
        }
    }
}
