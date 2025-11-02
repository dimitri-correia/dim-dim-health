use tests_helpers::test_server::get_app_state;
use uuid::Uuid;

#[tokio::test]
async fn test_user_repo_create_and_get() {
    let username = "testrepocreateuser";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let user_repo = get_app_state().await.user_repository;

    let res = user_repo
        .create(username, &email, password_hash)
        .await
        .unwrap();
    assert_eq!(res.username, username);
    assert_eq!(res.email, email);
    assert_eq!(res.password_hash, password_hash);
    assert_eq!(res.created_at, res.updated_at);
    assert!(!res.email_verified);

    let user_id = res.id;

    let res = user_repo.create(username, &email, password_hash).await;
    assert!(res.is_err());

    let res = user_repo.find_by_id(&user_id).await.unwrap();
    assert!(res.is_some());
    assert_eq!(res.unwrap().username, username);

    let res = user_repo.find_by_id(&Uuid::new_v4()).await.unwrap();
    assert!(res.is_none());

    let res = user_repo.find_by_email(&email).await.unwrap();
    assert!(res.is_some());
    assert_eq!(res.unwrap().username, username);

    let res = user_repo.find_by_email("notexisitingemail").await.unwrap();
    assert!(res.is_none());

    let res = user_repo.find_by_username(username).await.unwrap();
    assert!(res.is_some());
    assert_eq!(res.unwrap().email, email);

    let res = user_repo
        .find_by_username("notexistingusername")
        .await
        .unwrap();
    assert!(res.is_none());

    let res = user_repo
        .user_already_exists(&email, username)
        .await
        .unwrap();
    assert!(res);

    let res = user_repo
        .user_already_exists(&email, "notexistingusername")
        .await
        .unwrap();
    assert!(res);

    let res = user_repo
        .user_already_exists("notexistingemail", username)
        .await
        .unwrap();
    assert!(res);

    let res = user_repo
        .user_already_exists("notexistingemail", "notexistingusername")
        .await
        .unwrap();
    assert!(!res);
}

#[tokio::test]
async fn test_user_repo_create_and_update() {
    let username = "testrepoupdateuser";
    let email = format!("{username}@dimdim.fr");
    let password_hash = "securepassword";

    let user_repo = get_app_state().await.user_repository;
    let res = user_repo
        .create(username, &email, password_hash)
        .await
        .unwrap();

    let user_id = res.id;

    let res = user_repo.update(&user_id, None, None).await;
    assert!(res.is_err());

    let new_username = "updatedusername";
    let res = user_repo
        .update(&user_id, Some(new_username), None)
        .await
        .unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, email);

    let res = user_repo.find_by_id(&user_id).await.unwrap().unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, email);

    let new_email = format!("{new_username}@dimdim.fr");
    let res = user_repo
        .update(&user_id, None, Some(&new_email))
        .await
        .unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, new_email);

    let res = user_repo.find_by_id(&user_id).await.unwrap().unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, new_email);

    let final_username = "finalusername";
    let final_email = format!("{final_username}@dimdim.fr");
    let res = user_repo
        .update(&user_id, Some(final_username), Some(&final_email))
        .await
        .unwrap();
    assert_eq!(res.username, final_username);
    assert_eq!(res.email, final_email);

    let res = user_repo.find_by_id(&user_id).await.unwrap().unwrap();
    assert_eq!(res.username, final_username);
    assert_eq!(res.email, final_email);

    let res = user_repo
        .update(&Uuid::new_v4(), Some("nouser"), Some("aa@aa.fr"))
        .await;
    assert!(res.is_err());
}
