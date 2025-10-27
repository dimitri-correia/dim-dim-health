use crate::models::user::User;
use async_trait::async_trait;
use sqlx::Error as SqlxError;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, SqlxError>;

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, SqlxError>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, SqlxError>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, SqlxError>;

    async fn user_already_exists(&self, email: &str, username: &str) -> Result<bool, sqlx::Error>;

    async fn update(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        user_image: Option<&str>,
    ) -> Result<Option<User>, SqlxError>;
}
