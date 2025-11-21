use crate::helpers::test_server::get_app_state;
use uuid::Uuid;

#[tokio::test]
async fn test_user_repo_create_and_get() {
    let username = "testrepocreateuser";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    let res = user_repo
        .create(username, &email, password_hash, false)
        .await
        .unwrap();
    assert_eq!(res.username, username);
    assert_eq!(res.email, email);
    assert_eq!(res.password_hash, password_hash);
    assert_eq!(res.created_at, res.updated_at);
    assert!(!res.email_verified);

    let user_id = res.id;

    let res = user_repo.create(username, &email, password_hash, false).await;
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
    let username = "testrepoupdateuser";
    let email = format!("{username}@dimdim.fr");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;
    let res = user_repo
        .create(username, &email, password_hash, false)
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

#[tokio::test]
async fn test_sql_injection() {
    let email: &str = "attacker@attack.fr'); DROP TABLE users; --";
    let username = "test_sql_injection";
    let password_hash = "password";

    let user_repo = &get_app_state().await.repositories.user_repository;
    let res = user_repo
        .create(username, email, password_hash, false)
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
    let username = "testguestuser";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    let res = user_repo
        .create(username, &email, password_hash, true)
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
    let username = "testregularuser";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    let res = user_repo
        .create(username, &email, password_hash, false)
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

#[tokio::test]
async fn test_user_repo_partial_models() {
    let username = "testpartialuser";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let user_repo = &get_app_state().await.repositories.user_repository;

    // Create a test user
    let user = user_repo
        .create(username, &email, password_hash, false)
        .await
        .unwrap();
    let user_id = user.id;

    // Test find_by_email_for_auth - should only return auth fields
    let auth_model = user_repo
        .find_by_email_for_auth(&email)
        .await
        .unwrap()
        .expect("User should exist");
    assert_eq!(auth_model.id, user_id);
    assert_eq!(auth_model.email, email);
    assert_eq!(auth_model.password_hash, password_hash);
    assert_eq!(auth_model.email_verified, false);

    // Test find_by_id_public - should only return public fields
    let public_model = user_repo
        .find_by_id_public(&user_id)
        .await
        .unwrap()
        .expect("User should exist");
    assert_eq!(public_model.id, user_id);
    assert_eq!(public_model.username, username);
    assert_eq!(public_model.email, email);
    assert_eq!(public_model.email_verified, false);

    // Test find_by_email_basic - should only return basic fields
    let basic_model = user_repo
        .find_by_email_basic(&email)
        .await
        .unwrap()
        .expect("User should exist");
    assert_eq!(basic_model.id, user_id);
    assert_eq!(basic_model.username, username);
    assert_eq!(basic_model.email, email);

    // Test find_email_verification_status - should only return verification fields
    let verification_model = user_repo
        .find_email_verification_status(&user_id)
        .await
        .unwrap()
        .expect("User should exist");
    assert_eq!(verification_model.id, user_id);
    assert_eq!(verification_model.email, email);
    assert_eq!(verification_model.email_verified, false);

    // Test with non-existing user
    let non_existing_id = uuid::Uuid::new_v4();
    let result = user_repo
        .find_by_id_public(&non_existing_id)
        .await
        .unwrap();
    assert!(result.is_none());

    let result = user_repo
        .find_by_email_for_auth("nonexisting@test.fr")
        .await
        .unwrap();
    assert!(result.is_none());
}
