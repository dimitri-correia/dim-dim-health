use axum::http::StatusCode;
use serde_json::json;
use tests_helpers::{app_paths::APP_PATHS, test_server::get_test_server};

#[tokio::test]
async fn test_server_health() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server.get(APP_PATHS.health_check).await;
    res.assert_status(StatusCode::OK);
    res.assert_json(&json!(
        {
            "status":"ok"
        }
    ));
}

#[tokio::test]
async fn test_bad_route() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server.get("/non_existing_route").await;
    res.assert_status(StatusCode::NOT_FOUND);
}
