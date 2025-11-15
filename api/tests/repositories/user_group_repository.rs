use entities::sea_orm_active_enums::UserGroup;
use tests_helpers::test_server::get_app_state;
use uuid::Uuid;

#[tokio::test]
async fn test_without_valid_user() {
    let user_id = Uuid::new_v4();

    let app_state = get_app_state().await;

    let res = app_state
        .repositories
        .user_group_repository
        .create(&user_id, UserGroup::PublicGroup)
        .await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_user_group_repo_create_and_get() {
    let username = "testrepousergroup";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    let res = app_state
        .repositories
        .user_group_repository
        .create(&user.id, UserGroup::PublicGroup)
        .await
        .unwrap();

    assert_eq!(res.user_id, user.id);
    assert_eq!(res.group, UserGroup::PublicGroup);

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].group, UserGroup::PublicGroup);

    // Add another group
    let res = app_state
        .repositories
        .user_group_repository
        .create(&user.id, UserGroup::AdminGroup)
        .await
        .unwrap();

    assert_eq!(res.user_id, user.id);
    assert_eq!(res.group, UserGroup::AdminGroup);

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 2);
}

#[tokio::test]
async fn test_delete_by_user_id_and_group() {
    let username = "testdeleteusergroup";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    app_state
        .repositories
        .user_group_repository
        .create(&user.id, UserGroup::PublicGroup)
        .await
        .unwrap();

    app_state
        .repositories
        .user_group_repository
        .create(&user.id, UserGroup::AdminGroup)
        .await
        .unwrap();

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 2);

    app_state
        .repositories
        .user_group_repository
        .delete_by_user_id_and_group(&user.id, UserGroup::PublicGroup)
        .await
        .unwrap();

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].group, UserGroup::AdminGroup);

    // Delete non-existing
    app_state
        .repositories
        .user_group_repository
        .delete_by_user_id_and_group(&user.id, UserGroup::PublicGroup)
        .await
        .unwrap();

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 1);
}

#[tokio::test]
async fn test_delete() {
    let username = "testdeletemodelgroup";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    app_state
        .repositories
        .user_group_repository
        .create(&user.id, UserGroup::PublicGroup)
        .await
        .unwrap();

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 1);

    app_state
        .repositories
        .user_group_repository
        .delete(groups[0].clone())
        .await
        .unwrap();

    let groups = app_state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert_eq!(groups.len(), 0);
}
