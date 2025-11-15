use chrono::NaiveDate;
use entities::sea_orm_active_enums::GenderEnum;
use sea_orm::prelude::Decimal;
use std::str::FromStr;
use tests_helpers::test_server::get_app_state;
use uuid::Uuid;

#[tokio::test]
async fn test_without_valid_user() {
    let user_id = Uuid::new_v4();
    let birth_date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
    let height_in_cm = 170;
    let gender = GenderEnum::Male;
    let activity_level = Decimal::from_str("1.5").unwrap();

    let app_state = get_app_state().await;

    let res = app_state
        .repositories
        .user_info_repository
        .create(&user_id, &birth_date, height_in_cm, gender, activity_level)
        .await;

    assert!(res.is_err());
}

#[tokio::test]
async fn test_user_info_repo_create_and_get() {
    let username = "testrepouserinfo";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    let birth_date = NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
    let height_in_cm = 175;
    let gender = GenderEnum::Male;
    let activity_level = Decimal::from_str("1.5").unwrap();

    let res = app_state
        .repositories
        .user_info_repository
        .create(&user.id, &birth_date, height_in_cm, gender.clone(), activity_level.clone())
        .await
        .unwrap();

    assert_eq!(res.user_id, user.id);
    assert_eq!(res.birth_date, birth_date);
    assert_eq!(res.height_in_cm, height_in_cm);
    assert_eq!(res.gender, gender);
    assert_eq!(res.activity_level, activity_level);

    let res = app_state
        .repositories
        .user_info_repository
        .find_by_user_id(&user.id)
        .await
        .unwrap();

    assert!(res.is_some());
    let info = res.unwrap();
    assert_eq!(info.user_id, user.id);
    assert_eq!(info.birth_date, birth_date);
    assert_eq!(info.height_in_cm, height_in_cm);

    let res = app_state
        .repositories
        .user_info_repository
        .find_by_user_id(&Uuid::new_v4())
        .await
        .unwrap();

    assert!(res.is_none());
}

#[tokio::test]
async fn test_user_info_repo_update() {
    let username = "testupdateuserinfo";
    let email = format!("{username}@test.fr");
    let password_hash = "securepassword";

    let app_state = get_app_state().await;

    let user = app_state
        .repositories
        .user_repository
        .create(username, &email, password_hash)
        .await
        .unwrap();

    let birth_date = NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
    let height_in_cm = 175;
    let gender = GenderEnum::Male;
    let activity_level = Decimal::from_str("1.5").unwrap();

    app_state
        .repositories
        .user_info_repository
        .create(&user.id, &birth_date, height_in_cm, gender.clone(), activity_level.clone())
        .await
        .unwrap();

    // Update with no fields should fail
    let res = app_state
        .repositories
        .user_info_repository
        .update(&user.id, None, None, None, None)
        .await;

    assert!(res.is_err());

    // Update birth date
    let new_birth_date = NaiveDate::from_ymd_opt(1995, 10, 20).unwrap();
    let res = app_state
        .repositories
        .user_info_repository
        .update(&user.id, Some(&new_birth_date), None, None, None)
        .await
        .unwrap();

    assert_eq!(res.birth_date, new_birth_date);
    assert_eq!(res.height_in_cm, height_in_cm);

    // Update height
    let new_height = 180;
    let res = app_state
        .repositories
        .user_info_repository
        .update(&user.id, None, Some(new_height), None, None)
        .await
        .unwrap();

    assert_eq!(res.birth_date, new_birth_date);
    assert_eq!(res.height_in_cm, new_height);

    // Update gender
    let new_gender = GenderEnum::Female;
    let res = app_state
        .repositories
        .user_info_repository
        .update(&user.id, None, None, Some(new_gender.clone()), None)
        .await
        .unwrap();

    assert_eq!(res.gender, new_gender);
    assert_eq!(res.height_in_cm, new_height);

    // Update activity level
    let new_activity = Decimal::from_str("2.0").unwrap();
    let res = app_state
        .repositories
        .user_info_repository
        .update(&user.id, None, None, None, Some(new_activity.clone()))
        .await
        .unwrap();

    assert_eq!(res.activity_level, new_activity);
    assert_eq!(res.gender, new_gender);

    // Update multiple fields
    let final_birth_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let final_height = 165;
    let res = app_state
        .repositories
        .user_info_repository
        .update(
            &user.id,
            Some(&final_birth_date),
            Some(final_height),
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(res.birth_date, final_birth_date);
    assert_eq!(res.height_in_cm, final_height);
    assert_eq!(res.gender, new_gender);
    assert_eq!(res.activity_level, new_activity);
}
