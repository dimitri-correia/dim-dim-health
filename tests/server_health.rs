mod common;
use axum::http::StatusCode;
use common::test_server::get_test_server;
use serde_json::json;

#[tokio::test]
async fn test_server_health() {
    let server = get_test_server().await;
    let res = server.get("/health").await;
    res.assert_status(StatusCode::OK);
    res.assert_json(&json!(
        {
            "status":"ok"
        }
    ));
}

#[tokio::test]
async fn test_bad_route() {
    let server = get_test_server().await;
    let res = server.get("/non_existing_route").await;
    res.assert_status(StatusCode::NOT_FOUND);
}
