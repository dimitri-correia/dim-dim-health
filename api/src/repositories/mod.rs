use sea_orm::DatabaseConnection;

use crate::repositories::{
    email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
};

pub mod email_verification_repository;
pub mod user_repository;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: UserRepository,
    pub email_verification_repository: EmailVerificationRepository,
}

impl Repositories {
    pub fn new(db: DatabaseConnection) -> Self {
        let user_repository = UserRepository::new(db.clone());
        let email_verification_repository = EmailVerificationRepository::new(db.clone());

        Repositories {
            user_repository,
            email_verification_repository,
        }
    }
}
