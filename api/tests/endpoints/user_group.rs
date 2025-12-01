use crate::helpers::{
    app_paths::APP_PATHS,
    test_data::TestData,
    test_server::{get_app_state, get_test_server},
};
use axum::http::{HeaderValue, StatusCode};

#[tokio::test]
async fn test_join_public_group_already_member() {
    let td = TestData::with_base_name("publicgroupuser");
    let (user, access_token) = td.create_user_with_token().await;

    let app_test = get_app_state().await;
    let server = get_test_server(app_test.clone()).await;

    let user_id = user.id;

    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", access_token).as_str()).unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);

    // Try to join again (should still be OK - idempotent)
    let res = server
        .post(APP_PATHS.join_public_group)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", access_token).as_str())
                .unwrap(),
        )
        .await;

    res.assert_status(StatusCode::OK);

    let res = server
        .get(APP_PATHS.get_public_group_members)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", access_token).as_str())
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
        .get(APP_PATHS.get_user_groups)
        .add_header(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", access_token).as_str())
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
            HeaderValue::from_str(format!("Token {}", access_token).as_str())
                .unwrap(),
        )
        .await;
    res.assert_status(StatusCode::OK);
}
