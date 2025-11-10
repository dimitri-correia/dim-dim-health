use chrono::{DateTime, Duration, FixedOffset, Utc};
use once_cell::sync::Lazy;
use tests_helpers::test_server::get_app_state;
use uuid::Uuid;

static EXPIRES_AT: Lazy<DateTime<FixedOffset>> = Lazy::new(|| {
    let offset = FixedOffset::east_opt(0).unwrap();
    Utc::now().with_timezone(&offset) + Duration::days(2)
});

#[tokio::test]
async fn test_without_valid_user() {
    let user_id = Uuid::new_v4();
    let token = "token";

    let app_state = get_app_state().await;

    let res = app_state
        .repositories
        .email_verification_repository
        .create_token(&user_id, token, &EXPIRES_AT)
        .await;

    assert!(res.is_err());

    let res = app_state
        .repositories
        .email_verification_repository
        .verify_user_email(&user_id)
        .await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_email_verif_repo_create_and_get() {
    let username = "testrepoemailverif";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    assert!(!user.email_verified);

    let token = "token";

    let res = app_state
        .repositories
        .email_verification_repository
        .create_token(&user.id, token, &EXPIRES_AT)
        .await
        .unwrap();

    assert_eq!(res.token, token);
    assert_eq!(
        res.expires_at.timestamp_micros(),
        EXPIRES_AT.timestamp_micros()
    ); // db stores in micro but rust use nano
    assert_eq!(res.user_id, user.id);

    let res = app_state
        .repositories
        .email_verification_repository
        .find_by_token(token)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(res.user_id, user.id);
    assert_eq!(res.token, token);

    let res = app_state
        .repositories
        .email_verification_repository
        .delete_by_token(token)
        .await
        .unwrap();
    assert!(res);

    let res = app_state
        .repositories
        .email_verification_repository
        .find_by_token(token)
        .await
        .unwrap();
    assert!(res.is_none());

    // delete on non existing
    let res = app_state
        .repositories
        .email_verification_repository
        .delete_by_token(token)
        .await
        .unwrap();
    assert!(!res);

    // test expired token
    let expired_token = "expiredtoken";
    let expires_at = EXPIRES_AT.fixed_offset() - Duration::days(3);
    let res = app_state
        .repositories
        .email_verification_repository
        .create_token(&user.id, expired_token, &expires_at)
        .await
        .unwrap();

    assert_eq!(res.token, expired_token);

    let res = app_state
        .repositories
        .email_verification_repository
        .find_by_token(expired_token)
        .await
        .unwrap();

    assert!(res.is_none())
}
