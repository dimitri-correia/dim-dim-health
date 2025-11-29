use crate::helpers::{test_data::TestData, test_server::get_app_state};
use uuid::Uuid;

#[tokio::test]
async fn test_user_repo_create_and_get() {
    let td = TestData::new();
    let username = td.username("testrepocreateuser");
    let email = td.email("testrepocreateuser");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    let res = user_repo
        .create(&username, &email, password_hash, false)
        .await
        .unwrap();
    assert_eq!(res.username, username);
    assert_eq!(res.email, email);
    assert_eq!(res.password_hash, password_hash);
    assert_eq!(res.created_at, res.updated_at);
    assert!(!res.email_verified);

    let user_id = res.id;

    let res = user_repo
        .create(&username, &email, password_hash, false)
        .await;
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

    let res = user_repo.find_by_username(&username).await.unwrap();
    assert!(res.is_some());
    assert_eq!(res.unwrap().email, email);

    let res = user_repo
        .find_by_username("notexistingusername")
        .await
        .unwrap();
    assert!(res.is_none());

    let res = user_repo
        .user_already_exists(&email, &username)
        .await
        .unwrap();
    assert!(res);

    let res = user_repo
        .user_already_exists(&email, "notexistingusername")
        .await
        .unwrap();
    assert!(res);

    let res = user_repo
        .user_already_exists("notexistingemail", &username)
        .await
        .unwrap();
    assert!(res);

    let res = user_repo
        .user_already_exists("notexistingemail", "notexistingusername")
        .await
        .unwrap();
    assert!(!res);

    let new_secure_password = "newsecurepassword";
    let res = user_repo
        .update_password(&user_id, new_secure_password)
        .await
        .unwrap();
    assert_eq!(res.password_hash, new_secure_password);

    let res = user_repo.find_by_id(&user_id).await.unwrap();
    assert_eq!(res.unwrap().password_hash, new_secure_password);
}

#[tokio::test]
async fn test_user_repo_create_and_update() {
    let td = TestData::new();
    let username = td.username("testrepoupdateuser");
    let email = td.email("testrepoupdateuser");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;
    let res = user_repo
        .create(&username, &email, password_hash, false)
        .await
        .unwrap();

    let user_id = res.id;

    let res = user_repo.update(&user_id, None, None).await;
    assert!(res.is_err());

    let new_username = td.username("updatedusername");
    let res = user_repo
        .update(&user_id, Some(&new_username), None)
        .await
        .unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, email);

    let res = user_repo.find_by_id(&user_id).await.unwrap().unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, email);

    let new_email = td.email("updatedusername");
    let res = user_repo
        .update(&user_id, None, Some(&new_email))
        .await
        .unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, new_email);

    let res = user_repo.find_by_id(&user_id).await.unwrap().unwrap();
    assert_eq!(res.username, new_username);
    assert_eq!(res.email, new_email);

    let final_username = td.username("finalusername");
    let final_email = td.email("finalusername");
    let res = user_repo
        .update(&user_id, Some(&final_username), Some(&final_email))
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

#[tokio::test]
async fn test_sql_injection() {
    let td = TestData::new();
    let email = format!(
        "{}_attack'); DROP TABLE users; --@attack.fr",
        td.username("attacker")
    );
    let username = td.username("test_sql_injection");
    let password_hash = "password";

    let user_repo = &get_app_state().await.repositories.user_repository;
    let res = user_repo
        .create(&username, &email, password_hash, false)
        .await
        .unwrap();

    assert_eq!(res.username, username);
    assert_eq!(res.email, email);

    let res = user_repo.find_by_id(&res.id).await.unwrap().unwrap();
    assert_eq!(res.username, username);
    assert_eq!(res.email, email);
}

#[tokio::test]
async fn test_user_repo_create_guest_user() {
    let td = TestData::new();
    let username = td.username("testguestuser");
    let email = td.email("testguestuser");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    let res = user_repo
        .create(&username, &email, password_hash, true)
        .await
        .unwrap();
    assert_eq!(res.username, username);
    assert_eq!(res.email, email);
    assert_eq!(res.password_hash, password_hash);
    assert_eq!(res.created_at, res.updated_at);
    assert!(res.email_verified); // Guest users should have email_verified = true

    let user_id = res.id;

    let res = user_repo.find_by_id(&user_id).await.unwrap();
    assert!(res.is_some());
    let user = res.unwrap();
    assert_eq!(user.username, username);
    assert!(user.email_verified); // Verify it persists
}

#[tokio::test]
async fn test_user_repo_create_regular_user() {
    let td = TestData::new();
    let username = td.username("testregularuser");
    let email = td.email("testregularuser");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    let res = user_repo
        .create(&username, &email, password_hash, false)
        .await
        .unwrap();
    assert_eq!(res.username, username);
    assert_eq!(res.email, email);
    assert_eq!(res.password_hash, password_hash);
    assert_eq!(res.created_at, res.updated_at);
    assert!(!res.email_verified); // Regular users should have email_verified = false

    let user_id = res.id;

    let res = user_repo.find_by_id(&user_id).await.unwrap();
    assert!(res.is_some());
    let user = res.unwrap();
    assert_eq!(user.username, username);
    assert!(!user.email_verified); // Verify it persists
}
