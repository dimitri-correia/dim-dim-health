use axum::http::StatusCode;
use crate::helpers::{
    test_server::{get_app_state, get_test_server},
};

#[tokio::test]
async fn test_metrics_endpoint() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    // Make a request to ensure some metrics are collected
    let _ = server.get("/health").await;

    // Request metrics endpoint
    let res = server.get("/metrics").await;
    res.assert_status(StatusCode::OK);

    let body = res.text();
    
    // Verify Prometheus format is returned
    assert!(body.contains("http_requests_total"), "Expected http_requests_total metric");
    assert!(body.contains("http_request_duration_seconds"), "Expected http_request_duration_seconds metric");
    
    // Verify the health check request was tracked
    assert!(body.contains("method=\"GET\""), "Expected method label");
    assert!(body.contains("path=\"/health\""), "Expected path label");
}

#[tokio::test]
async fn test_metrics_tracks_requests() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    // Make several requests
    let _ = server.get("/health").await;
    let _ = server.get("/health").await;
    let _ = server.get("/health").await;

    // Get metrics
    let res = server.get("/metrics").await;
    res.assert_status(StatusCode::OK);

    let body = res.text();
    
    // Verify metrics are being tracked
    assert!(body.contains("http_requests_total"), "Expected http_requests_total metric");
    assert!(body.contains("path=\"/health\""), "Expected /health path to be tracked");
}

#[tokio::test]
async fn test_metrics_tracks_404() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    // Make a request to a non-existent route
    let _ = server.get("/non_existing_route").await;

    // Get metrics
    let res = server.get("/metrics").await;
    res.assert_status(StatusCode::OK);

    let body = res.text();
    
    // Verify 404 status is tracked
    assert!(body.contains("status=\"404\""), "Expected 404 status to be tracked");
}
