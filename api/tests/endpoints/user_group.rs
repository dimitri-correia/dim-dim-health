use crate::helpers::{
    app_paths::APP_PATHS,
    test_server::{get_app_state, get_test_server},
};
use axum::http::{HeaderValue, StatusCode};
use dimdim_health_api::schemas::auth_schemas::LoginResponse;
use serde_json::json;

#[tokio::test]
async fn test_join_public_group_already_member() {
    let username = "testalreadymember";
    let email = format!("{username}@dimdim.fr");
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

    let user_id = app_test
        .repositories
        .user_repository
        .find_by_username(username)
        .await
        .unwrap()
        .unwrap()
        .id;

    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);

    // Try to join again
    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);

    let res = server
        .get(APP_PATHS.get_public_group_members)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let members_response = res.json::<serde_json::Value>();
    let members = members_response
        .get("users")
        .and_then(|u| u.as_array())
        .unwrap();
    assert!(
        members
            .iter()
            .any(|m| m.as_str().unwrap() == user_id.to_string())
    );

    let res = server
        .post(APP_PATHS.get_user_groups)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let groups_response = res.json::<serde_json::Value>();
    let groups = groups_response
        .get("groups")
        .and_then(|g| g.as_array())
        .unwrap();
    assert!(groups.iter().any(|g| g.as_str().unwrap() == "PublicGroup"));

    let res = server
        .post(APP_PATHS.leave_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;
    res.assert_status(StatusCode::OK);
}
