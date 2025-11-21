use anyhow::Result;
use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::{HttpExporterBuilder, WithExportConfig};
use opentelemetry_sdk::{
    trace::{self, RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

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

    // If no OTLP endpoint is provided, use standard tracing without OpenTelemetry
    if otlp_endpoint.is_none() || otlp_endpoint == Some("") {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
        return Ok(());
    }

    let endpoint = otlp_endpoint.unwrap();

    // Create OpenTelemetry exporter
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .with_timeout(std::time::Duration::from_secs(3))
        .build()?;

    // Create tracer provider
    let provider = TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_config(
            trace::Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", service_name.to_string()),
                ])),
        )
        .build();

    // Set as global provider
    global::set_tracer_provider(provider.clone());

    // Create telemetry layer
    let tracer_name = service_name.to_string();
    let telemetry = tracing_opentelemetry::layer().with_tracer(provider.tracer(tracer_name));

    // Initialize tracing subscriber with both console and OpenTelemetry layers
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_filter(EnvFilter::new(env_filter)))
        .with(telemetry)
        .init();

    Ok(())
}

/// Shutdown OpenTelemetry gracefully
/// 
/// This should be called before the application exits to ensure all traces are flushed
pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}
