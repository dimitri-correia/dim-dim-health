use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use uuid::Uuid;

use crate::entities::email_verification_token;

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
        email_verification_token::Entity::find()
            .filter(email_verification_token::Column::Token.eq(token))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_token(&self, token: &str) -> Result<(), sea_orm::DbErr> {
        let token = email_verification_token::Entity::find()
            .filter(email_verification_token::Column::Token.eq(token))
            .one(&self.db)
            .await?;

        if let Some(token) = token {
            let active_model: email_verification_token::ActiveModel = token.into();
            active_model.delete(&self.db).await?;
        }

        Ok(())
    }

    pub async fn verify_user_email(&self, user_id: &Uuid) -> Result<(), sea_orm::DbErr> {
        let user = crate::entities::users::Entity::find_by_id(user_id.to_owned())
            .one(&self.db)
            .await?;

        if let Some(user) = user {
            let mut active_model: crate::entities::users::ActiveModel = user.into();
            active_model.email_verified = Set(true);
            active_model.update(&self.db).await?;
        }

        Ok(())
    }
}
