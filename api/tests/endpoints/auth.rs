use crate::helpers::{
    app_paths::APP_PATHS,
    test_data::TestData,
    test_server::{get_app_state, get_test_server},
};
use axum::http::{HeaderValue, StatusCode};
use dimdim_health_api::schemas::auth_schemas::{LoginResponse, UserResponse};
use serde_json::json;

#[tokio::test]
async fn test_create_user() {
    let td = TestData::new();
    let username = td.username("testcreateuser");
    let email = td.email("testcreateuser");
    let password = "securepassword";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server
        .post(APP_PATHS.create_user)
        .json(&json!({
            "user": {
                "username": username,
                "email": email,
                "password": password
            }
        }))
        .await;

    res.assert_status(StatusCode::OK);

    let login_response = res.json::<LoginResponse>();
    assert_eq!(login_response.user.username, username);
    assert_eq!(login_response.user.email, email);

    let res = server
        .get(APP_PATHS.current_user)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let current_user_data = res.json::<UserResponse>().user;
    assert_eq!(current_user_data.username, username);
    assert_eq!(current_user_data.email, email);

    let res = server
        .post(APP_PATHS.login_user)
        .json(&json!({
            "user": {
                "email": email,
                "password": password
            }
        }))
        .await;

    res.assert_status(StatusCode::OK);
    let login_user_data = res.json::<UserResponse>().user;
    assert_eq!(login_user_data.username, username);
    assert_eq!(login_user_data.email, email);
}

#[tokio::test]
async fn test_create_user_too_small_username() {
    let td = TestData::new();
    let username = "t";
    let email = td.email("t");
    let password = "securepassword";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server
        .post(APP_PATHS.create_user)
        .json(&json!({
                "user":{
                    "username":username,
                    "email":email,
                    "password":password
                }
        }))
        .await;
    res.assert_status(StatusCode::BAD_REQUEST);
    res.assert_json(&json!({"error":"username: Username must be between 3 and 20 characters"}));
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    let username = "testinvalidemail";
    let email = "invalid-email-format";
    let password = "securepassword";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server
        .post(APP_PATHS.create_user)
        .json(&json!({
                "user":{
                    "username":username,
                    "email":email,
                    "password":password
                }
        }))
        .await;
    res.assert_status(StatusCode::BAD_REQUEST);
    res.assert_json(&json!({"error":"email: Invalid email format"}));
}

#[tokio::test]
async fn test_create_user_weak_password() {
    let td = TestData::new();
    let username = td.username("testweakpassword");
    let email = td.email("testweakpassword");
    let password = "123";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server
        .post(APP_PATHS.create_user)
        .json(&json!({
                "user":{
                    "username":username,
                    "email":email,
                    "password":password
                }
        }))
        .await;
    res.assert_status(StatusCode::BAD_REQUEST);
    res.assert_json(&json!({"error":"password: Password must be at least 8 characters"}));
}

#[tokio::test]
async fn test_create_user_duplicate_username() {
    let td = TestData::new();
    let username = td.username("duplicateuser");
    let email1 = td.email("duplicateuser1");
    let email2 = td.email("duplicateuser2");
    let password = "securepassword";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res1 = server
        .post(APP_PATHS.create_user)
        .json(&json!({
                "user":{
                    "username":username,
                    "email":email1,
                    "password":password
                }
        }))
        .await;
    res1.assert_status(StatusCode::OK);

    let res2 = server
        .post(APP_PATHS.create_user)
        .json(&json!({
                "user":{
                    "username":username,
                    "email":email2,
                    "password":password
                }
        }))
        .await;
    res2.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_create_guest_user() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server.post(APP_PATHS.create_guest_user).await;

    res.assert_status(StatusCode::OK);

    let login_response = res.json::<LoginResponse>();
    assert!(login_response.user.username.starts_with("guest_"));
    assert!(login_response.user.email.ends_with("@dimdim.guest"));
    assert!(login_response.user.email_verified); // Guest users should have verified email

    // Verify guest user can access current_user endpoint
    let res = server
        .get(APP_PATHS.current_user)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let current_user_data = res.json::<UserResponse>().user;
    assert!(current_user_data.username.starts_with("guest_"));
    assert!(current_user_data.email.ends_with("@dimdim.guest"));
    assert!(current_user_data.email_verified);
}
