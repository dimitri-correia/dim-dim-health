use tests_helpers::test_server::get_app_state;
use uuid::Uuid;

#[tokio::test]
async fn test_without_valid_users() {
    let user_watched_id = Uuid::new_v4();
    let user_watching_id = Uuid::new_v4();

    let app_state = get_app_state().await;

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .create(&user_watched_id, &user_watching_id)
        .await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_user_watch_permission_repo_create_and_get() {
    let username1 = "testrepowatchperm1";
    let email1 = format!("{username1}@test.fr");
    let username2 = "testrepowatchperm2";
    let email2 = format!("{username2}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user1 = app_state
        .repositories
        .user_repository
        .create(username1, &email1, password_hash)
        .await
        .unwrap();

    let user2 = app_state
        .repositories
        .user_repository
        .create(username2, &email2, password_hash)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .create(&user1.id, &user2.id)
        .await
        .unwrap();

    assert_eq!(res.user_watched_id, user1.id);
    assert_eq!(res.user_watching_id, user2.id);

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap();

    assert!(res.is_some());
    let perm = res.unwrap();
    assert_eq!(perm.user_watched_id, user1.id);
    assert_eq!(perm.user_watching_id, user2.id);

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user2.id, &user1.id)
        .await
        .unwrap();

    assert!(res.is_none());
}

#[tokio::test]
async fn test_find_all_watched_and_watching() {
    let username1 = "testallwatched1";
    let email1 = format!("{username1}@test.fr");
    let username2 = "testallwatched2";
    let email2 = format!("{username2}@test.fr");
    let username3 = "testallwatched3";
    let email3 = format!("{username3}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user1 = app_state
        .repositories
        .user_repository
        .create(username1, &email1, password_hash)
        .await
        .unwrap();

    let user2 = app_state
        .repositories
        .user_repository
        .create(username2, &email2, password_hash)
        .await
        .unwrap();

    let user3 = app_state
        .repositories
        .user_repository
        .create(username3, &email3, password_hash)
        .await
        .unwrap();

    // user2 and user3 are watching user1
    app_state
        .repositories
        .user_watch_permission_repository
        .create(&user1.id, &user2.id)
        .await
        .unwrap();

    app_state
        .repositories
        .user_watch_permission_repository
        .create(&user1.id, &user3.id)
        .await
        .unwrap();

    // user1 is watching user2
    app_state
        .repositories
        .user_watch_permission_repository
        .create(&user2.id, &user1.id)
        .await
        .unwrap();

    // Find all watching user1
    let watched = app_state
        .repositories
        .user_watch_permission_repository
        .find_all_watched(&user1.id)
        .await
        .unwrap();

    assert_eq!(watched.len(), 2);

    // Find all user1 is watching
    let watching = app_state
        .repositories
        .user_watch_permission_repository
        .find_all_watching(&user1.id)
        .await
        .unwrap();

    assert_eq!(watching.len(), 1);
    assert_eq!(watching[0].user_watched_id, user2.id);
}

#[tokio::test]
async fn test_delete_by_user_ids() {
    let username1 = "testdeletewatchperm1";
    let email1 = format!("{username1}@test.fr");
    let username2 = "testdeletewatchperm2";
    let email2 = format!("{username2}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user1 = app_state
        .repositories
        .user_repository
        .create(username1, &email1, password_hash)
        .await
        .unwrap();

    let user2 = app_state
        .repositories
        .user_repository
        .create(username2, &email2, password_hash)
        .await
        .unwrap();

    app_state
        .repositories
        .user_watch_permission_repository
        .create(&user1.id, &user2.id)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap();

    assert!(res.is_some());

    app_state
        .repositories
        .user_watch_permission_repository
        .delete_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap();

    assert!(res.is_none());

    // Delete non-existing should not fail
    app_state
        .repositories
        .user_watch_permission_repository
        .delete_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_delete() {
    let username1 = "testdeletemodel1";
    let email1 = format!("{username1}@test.fr");
    let username2 = "testdeletemodel2";
    let email2 = format!("{username2}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user1 = app_state
        .repositories
        .user_repository
        .create(username1, &email1, password_hash)
        .await
        .unwrap();

    let user2 = app_state
        .repositories
        .user_repository
        .create(username2, &email2, password_hash)
        .await
        .unwrap();

    app_state
        .repositories
        .user_watch_permission_repository
        .create(&user1.id, &user2.id)
        .await
        .unwrap();

    let perm = app_state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap()
        .unwrap();

    app_state
        .repositories
        .user_watch_permission_repository
        .delete(perm)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user1.id, &user2.id)
        .await
        .unwrap();

    assert!(res.is_none());
}
