mod common;
use axum::http::{HeaderValue, StatusCode};
use common::{app_paths::APP_PATHS, test_server::get_test_server};
use dimdim_health::schemas::auth_schemas::UserResponse;
use serde_json::json;

#[tokio::test]
async fn test_create_user() {
    let username = "testcreateuser";
    let email = format!("{username}@dimdim.fr");
    let password = "securepassword";

    let server = get_test_server().await;

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

    let user_data = res.json::<UserResponse>().user;
    assert_eq!(user_data.username, username);
    assert_eq!(user_data.email, email);

    let res = server
        .get(APP_PATHS.current_user)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user_data.token).as_str()).unwrap(),
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
    let username = "t";
    let email = format!("{username}@dimdim.fr");
    let password = "securepassword";

    let server = get_test_server().await;
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

    let server = get_test_server().await;
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
    let username = "testweakpassword";
    let email = format!("{username}@email.fr");
    let password = "123";

    let server = get_test_server().await;
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
    let username = "duplicateuser";
    let email1 = format!("{username}1@dimdim.fr");
    let email2 = format!("{username}2@dimdim.fr");
    let password = "securepassword";

    let server = get_test_server().await;

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
