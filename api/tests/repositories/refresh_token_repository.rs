use tests_helpers::test_server::get_app_state;
use uuid::Uuid;

#[tokio::test]
async fn test_without_valid_user() {
    let user_id = Uuid::new_v4();
    let token = "refreshtoken";

    let app_state = get_app_state().await;

    let res = app_state
        .repositories
        .refresh_token_repository
        .create_token(&user_id, token)
        .await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_refresh_token_repo_create_and_get() {
    let username = "testreporefreshtoken";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    let token = "refreshtoken";

    let res = app_state
        .repositories
        .refresh_token_repository
        .create_token(&user.id, token)
        .await
        .unwrap();

    assert_eq!(res.token, token);
    assert_eq!(res.user_id, user.id);
    assert!(res.used_at.is_none());

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(res.user_id, user.id);
    assert_eq!(res.token, token);
    assert!(res.used_at.is_none());

    // Mark as used
    app_state
        .repositories
        .refresh_token_repository
        .mark_token_as_used(token)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token)
        .await
        .unwrap()
        .unwrap();

    assert!(res.used_at.is_some());

    let res = app_state
        .repositories
        .refresh_token_repository
        .delete_by_token(token)
        .await
        .unwrap();
    assert!(res);

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token)
        .await
        .unwrap();
    assert!(res.is_none());

    // delete on non existing
    let res = app_state
        .repositories
        .refresh_token_repository
        .delete_by_token(token)
        .await
        .unwrap();
    assert!(!res);
}

#[tokio::test]
async fn test_delete_all_user_tokens() {
    let username = "testdeleteallrefreshtoken";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    let token1 = "refreshtoken1";
    let token2 = "refreshtoken2";

    app_state
        .repositories
        .refresh_token_repository
        .create_token(&user.id, token1)
        .await
        .unwrap();

    app_state
        .repositories
        .refresh_token_repository
        .create_token(&user.id, token2)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token1)
        .await
        .unwrap();
    assert!(res.is_some());

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token2)
        .await
        .unwrap();
    assert!(res.is_some());

    let res = app_state
        .repositories
        .refresh_token_repository
        .delete_all_user_tokens(&user.id)
        .await
        .unwrap();
    assert!(res);

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token1)
        .await
        .unwrap();
    assert!(res.is_none());

    let res = app_state
        .repositories
        .refresh_token_repository
        .find_by_token(token2)
        .await
        .unwrap();
    assert!(res.is_none());
}
