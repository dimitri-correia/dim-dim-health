use axum::http::Request;
use tower_http::trace::{MakeSpan, OnRequest, OnResponse};
use tracing::{Level, Span};
use uuid::Uuid;

/// Creates a span for each request with a unique request ID
#[derive(Clone)]
pub struct MakeRequestSpan;

impl<B> MakeSpan<B> for MakeRequestSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = Uuid::new_v4();
        
        tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            request_id = %request_id,
        )
    }
}

/// Log when a request is received
#[derive(Clone)]
pub struct OnRequestLog;

impl<B> OnRequest<B> for OnRequestLog {
    fn on_request(&mut self, _request: &Request<B>, _span: &Span) {
        tracing::info!("started processing request");
    }
}

/// Log when a response is sent
#[derive(Clone)]
pub struct OnResponseLog;

impl<B> OnResponse<B> for OnResponseLog {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        _span: &Span,
    ) {
        let status = response.status();
        let latency_ms = latency.as_millis();
        
        if status.is_server_error() {
            tracing::error!(
                status = %status,
                latency_ms = %latency_ms,
                "request completed with error"
            );
        } else if status.is_client_error() {
            tracing::warn!(
                status = %status,
                latency_ms = %latency_ms,
                "request completed with client error"
            );
        } else {
            tracing::info!(
                status = %status,
                latency_ms = %latency_ms,
                "request completed successfully"
            );
        }
    }
}
