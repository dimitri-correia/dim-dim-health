use chrono::Utc;
use entities::{email_verification_token, token_partial::EmailVerificationTokenValidationModel, users};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct EmailVerificationRepository {
    db: DatabaseConnection,
}

impl EmailVerificationRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_token(
        &self,
        user_id: &Uuid,
        token: &str,
        expires_at: &chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<email_verification_token::Model, sea_orm::DbErr> {
        let email_verification_token = email_verification_token::ActiveModel {
            id: NotSet,
            user_id: Set(user_id.to_owned()),
            token: Set(token.to_owned()),
            expires_at: Set(expires_at.to_owned()),
            created_at: NotSet,
        };
        let email_verification_token = email_verification_token.insert(&self.db).await?;

        Ok(email_verification_token)
    }

    pub async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<email_verification_token::Model>, sea_orm::DbErr> {
        // Note : the expired token won't be returned
        let now = Utc::now();
        email_verification_token::Entity::find()
            .filter(email_verification_token::Column::Token.eq(token))
            .filter(email_verification_token::Column::ExpiresAt.gte(now))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_token(&self, token: &str) -> Result<bool, sea_orm::DbErr> {
        let token = email_verification_token::Entity::find()
            .filter(email_verification_token::Column::Token.eq(token))
            .one(&self.db)
            .await?;

        if let Some(token) = token {
            let active_model: email_verification_token::ActiveModel = token.into();
            active_model.delete(&self.db).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn verify_user_email(&self, user_id: &Uuid) -> Result<users::Model, sea_orm::DbErr> {
        let mut active = users::ActiveModel {
            id: Set(user_id.to_owned()),
            ..Default::default()
        };

        active.email_verified = Set(true);

        active.update(&self.db).await
    }

    // Partial model queries for optimized database access

    /// Find token for validation - returns only fields needed for validation
    pub async fn find_by_token_for_validation(
        &self,
        token: &str,
    ) -> Result<Option<EmailVerificationTokenValidationModel>, sea_orm::DbErr> {
        // Note: the expired token won't be returned
        let now = Utc::now();
        email_verification_token::Entity::find()
            .filter(email_verification_token::Column::Token.eq(token))
            .filter(email_verification_token::Column::ExpiresAt.gte(now))
            .select_only()
            .column(email_verification_token::Column::UserId)
            .column(email_verification_token::Column::ExpiresAt)
            .into_model::<EmailVerificationTokenValidationModel>()
            .one(&self.db)
            .await
    }
}
