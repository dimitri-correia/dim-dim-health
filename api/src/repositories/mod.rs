use sea_orm::DatabaseConnection;

use crate::repositories::{
    email_verification_repository::EmailVerificationRepository,
    food_item_repository::FoodItemRepository,
    meal_item_repository::MealItemRepository,
    meal_repository::MealRepository,
    password_reset_repository::PasswordResetRepository,
    refresh_token_repository::RefreshTokenRepository, user_group_repository::UserGroupsRepository,
    user_info_repository::UserInfoRepository, user_repository::UserRepository,
    user_watch_permission_repository::UserWatchPermissionRepository,
    user_weight_repository::UserWeightRepository,
};

pub mod email_verification_repository;
pub mod food_item_repository;
pub mod meal_item_repository;
pub mod meal_repository;
pub mod password_reset_repository;
pub mod refresh_token_repository;
pub mod user_group_repository;
pub mod user_info_repository;
pub mod user_repository;
pub mod user_watch_permission_repository;
pub mod user_weight_repository;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: UserRepository,
    pub email_verification_repository: EmailVerificationRepository,
    pub password_reset_repository: PasswordResetRepository,
    pub refresh_token_repository: RefreshTokenRepository,
    pub user_info_repository: UserInfoRepository,
    pub user_group_repository: UserGroupsRepository,
    pub user_watch_permission_repository: UserWatchPermissionRepository,
    pub user_weight_repository: UserWeightRepository,
    pub meal_repository: MealRepository,
    pub meal_item_repository: MealItemRepository,
    pub food_item_repository: FoodItemRepository,
}

impl Repositories {
    pub fn new(db: DatabaseConnection) -> Self {
        let user_repository = UserRepository::new(db.clone());
        let email_verification_repository = EmailVerificationRepository::new(db.clone());
        let password_reset_repository = PasswordResetRepository::new(db.clone());
        let refresh_token_repository = RefreshTokenRepository::new(db.clone());
        let user_info_repository = UserInfoRepository::new(db.clone());
        let user_group_repository = UserGroupsRepository::new(db.clone());
        let user_watch_permission_repository = UserWatchPermissionRepository::new(db.clone());
        let user_weight_repository = UserWeightRepository::new(db.clone());
        let meal_repository = MealRepository::new(db.clone());
        let meal_item_repository = MealItemRepository::new(db.clone());
        let food_item_repository = FoodItemRepository::new(db.clone());

        Self {
            user_repository,
            email_verification_repository,
            password_reset_repository,
            refresh_token_repository,
            user_info_repository,
            user_group_repository,
            user_watch_permission_repository,
            user_weight_repository,
            meal_repository,
            meal_item_repository,
            food_item_repository,
        }
    }
}
