use crate::helpers::{
    app_paths::APP_PATHS,
    test_server::{get_app_state, get_test_server},
};
use axum::http::{HeaderValue, StatusCode};
use dimdim_health_api::schemas::auth_schemas::LoginResponse;
use dimdim_health_api::schemas::user_watch_permission_schemas::{
    SearchUsersResponse, WatchersResponse, WatchingResponse,
};
use serde_json::json;
use uuid::Uuid;

// Helper function to create a user and return their token
async fn create_test_user(username: &str, email: &str) -> (LoginResponse, axum_test::TestServer) {
    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let res = server
        .post(APP_PATHS.create_user)
        .json(&json!({
            "user": {
                "username": username,
                "email": email,
                "password": "securepassword123"
            }
        }))
        .await;

    res.assert_status(StatusCode::OK);
    let login_response = res.json::<LoginResponse>();
    (login_response, server)
}

// Helper function to get a user's ID by searching for them
async fn get_user_id_by_username(
    server: &axum_test::TestServer,
    token: &str,
    username: &str,
) -> Uuid {
    let res = server
        .get(&format!("/api/users/search?query={}", username))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let search_response = res.json::<SearchUsersResponse>();
    search_response
        .users
        .iter()
        .find(|u| u.username == username)
        .expect("User not found in search results")
        .id
}

#[tokio::test]
async fn test_search_users() {
    // Create test users
    let (_user1, _) = create_test_user("searchtest1", "searchtest1@dimdim.fr").await;
    let (user2, server2) = create_test_user("searchtest2", "searchtest2@dimdim.fr").await;
    let (_user3, _) = create_test_user("searchtest3", "searchtest3@dimdim.fr").await;

    // Search for users with query "searchtest"
    let res = server2
        .get("/api/users/search?query=searchtest")
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user2.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let search_response = res.json::<SearchUsersResponse>();
    
    // Should find at least 2 users (user1 and user3), excluding the current user (user2)
    assert!(search_response.users.len() >= 2);
    
    // Verify the current user is not in the results
    assert!(!search_response.users.iter().any(|u| u.username == "searchtest2"));
}

#[tokio::test]
async fn test_search_users_too_short() {
    let (user1, server) = create_test_user("shortsearchtest", "shortsearchtest@dimdim.fr").await;

    // Search with query less than 3 characters should fail
    let res = server
        .get("/api/users/search?query=ab")
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_grant_and_get_watch_permissions() {
    // Create two test users
    let (user1, server1) = create_test_user("watchuser1", "watchuser1@dimdim.fr").await;
    let (user2, server2) = create_test_user("watchuser2", "watchuser2@dimdim.fr").await;

    // Get user2's ID by searching
    let user2_id = get_user_id_by_username(&server1, &user1.access_token, "watchuser2").await;

    // User1 grants permission to User2 (User2 can watch User1)
    let res = server1
        .post("/api/watch-permissions/grant")
        .json(&json!({
            "user_id": user2_id
        }))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::CREATED);

    // User1 gets their watchers (should include User2)
    let res = server1
        .get("/api/watch-permissions/watchers")
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let watchers_response = res.json::<WatchersResponse>();
    assert_eq!(watchers_response.watchers.len(), 1);
    assert_eq!(watchers_response.watchers[0].username, "watchuser2");

    // User2 gets who they are watching (should include User1)
    let res = server2
        .get("/api/watch-permissions/watching")
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user2.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let watching_response = res.json::<WatchingResponse>();
    assert_eq!(watching_response.watching.len(), 1);
    assert_eq!(watching_response.watching[0].username, "watchuser1");
}

#[tokio::test]
async fn test_revoke_watch_permission() {
    // Create two test users
    let (user1, server1) = create_test_user("revokeuser1", "revokeuser1@dimdim.fr").await;
    let (_user2, _) = create_test_user("revokeuser2", "revokeuser2@dimdim.fr").await;

    // Get user2's ID by searching
    let user2_id = get_user_id_by_username(&server1, &user1.access_token, "revokeuser2").await;

    // User1 grants permission to User2
    let res = server1
        .post("/api/watch-permissions/grant")
        .json(&json!({
            "user_id": user2_id
        }))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::CREATED);

    // User1 revokes permission from User2
    let res = server1
        .post("/api/watch-permissions/revoke")
        .json(&json!({
            "user_id": user2_id
        }))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);

    // User1 gets their watchers (should be empty now)
    let res = server1
        .get("/api/watch-permissions/watchers")
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);
    let watchers_response = res.json::<WatchersResponse>();
    assert_eq!(watchers_response.watchers.len(), 0);
}

#[tokio::test]
async fn test_grant_duplicate_permission() {
    // Create two test users
    let (user1, server1) = create_test_user("dupuser1", "dupuser1@dimdim.fr").await;
    let (_user2, _) = create_test_user("dupuser2", "dupuser2@dimdim.fr").await;

    // Get user2's ID by searching
    let user2_id = get_user_id_by_username(&server1, &user1.access_token, "dupuser2").await;

    // User1 grants permission to User2
    let res = server1
        .post("/api/watch-permissions/grant")
        .json(&json!({
            "user_id": user2_id
        }))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::CREATED);

    // User1 tries to grant permission to User2 again (should fail with conflict)
    let res = server1
        .post("/api/watch-permissions/grant")
        .json(&json!({
            "user_id": user2_id
        }))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_revoke_nonexistent_permission() {
    // Create two test users
    let (user1, server1) = create_test_user("nonexistuser1", "nonexistuser1@dimdim.fr").await;
    let (_user2, _) = create_test_user("nonexistuser2", "nonexistuser2@dimdim.fr").await;

    // Get user2's ID by searching
    let user2_id = get_user_id_by_username(&server1, &user1.access_token, "nonexistuser2").await;

    // User1 tries to revoke permission from User2 without granting first
    let res = server1
        .post("/api/watch-permissions/revoke")
        .json(&json!({
            "user_id": user2_id
        }))
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", user1.access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::NOT_FOUND);
}
