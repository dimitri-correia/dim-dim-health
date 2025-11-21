use crate::helpers::{
    app_paths::APP_PATHS,
    test_server::{get_app_state, get_test_server},
};
use axum::http::{HeaderValue, StatusCode};
use dimdim_health_api::schemas::{
    auth_schemas::LoginResponse, user_group_schemas::JoinPublicGroupResponse,
};
use serde_json::json;

#[tokio::test]
async fn test_join_public_group_success() {
    let username = "testjoinpublic";
    let email = format!("{username}@dimdim.fr");
    let password = "securepassword";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    // Create a new user
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

    // Join the public group
    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let join_response = res.json::<JoinPublicGroupResponse>();
    assert_eq!(
        join_response.message,
        "Successfully joined the public group"
    );

    // Verify user is in the public group by checking in database
    // We need to get the user ID properly
    let user = app_test
        .repositories
        .user_repository
        .find_by_email(&email)
        .await
        .unwrap()
        .unwrap();

    let user_groups = app_test
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert!(
        user_groups
            .iter()
            .any(|ug| ug.group == entities::sea_orm_active_enums::UserGroup::PublicGroup),
        "User should be in the public group"
    );
}

#[tokio::test]
async fn test_join_public_group_already_member() {
    let username = "testalreadymember";
    let email = format!("{username}@dimdim.fr");
    let password = "securepassword";

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    // Create a new user
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

    // Join the public group for the first time
    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let join_response = res.json::<JoinPublicGroupResponse>();
    assert_eq!(
        join_response.message,
        "Successfully joined the public group"
    );

    // Try to join again (should get "already a member" message)
    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", login_response.access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let join_response = res.json::<JoinPublicGroupResponse>();
    assert_eq!(
        join_response.message,
        "You are already a member of the public group"
    );
}

#[tokio::test]
async fn test_join_public_group_unauthorized() {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    // Try to join without authentication
    let res = server.post(APP_PATHS.join_public_group).await;

    res.assert_status(StatusCode::UNAUTHORIZED);
}
