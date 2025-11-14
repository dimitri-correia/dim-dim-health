use chrono::Utc;
use entities::password_reset_token;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct PasswordResetRepository {
    db: DatabaseConnection,
}

impl PasswordResetRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_token(
        &self,
        user_id: &Uuid,
        token: &str,
        expires_at: &chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<password_reset_token::Model, sea_orm::DbErr> {
        let password_reset_token = password_reset_token::ActiveModel {
            id: NotSet,
            user_id: Set(user_id.to_owned()),
            token: Set(token.to_owned()),
            expires_at: Set(expires_at.to_owned()),
            created_at: NotSet,
        };
        let password_reset_token = password_reset_token.insert(&self.db).await?;

        Ok(password_reset_token)
    }

    pub async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<password_reset_token::Model>, sea_orm::DbErr> {
        // Note : the expired token won't be returned
        let now = Utc::now();
        password_reset_token::Entity::find()
            .filter(password_reset_token::Column::Token.eq(token))
            .filter(password_reset_token::Column::ExpiresAt.gte(now))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_token(&self, token: &str) -> Result<bool, sea_orm::DbErr> {
        let token = password_reset_token::Entity::find()
            .filter(password_reset_token::Column::Token.eq(token))
            .one(&self.db)
            .await?;

        if let Some(token) = token {
            let active_model: password_reset_token::ActiveModel = token.into();
            active_model.delete(&self.db).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn delete_all_user_tokens(&self, user_id: &Uuid) -> Result<bool, sea_orm::DbErr> {
        password_reset_token::Entity::delete_many()
            .filter(password_reset_token::Column::UserId.eq(*user_id))
            .exec(&self.db)
            .await?;

        Ok(true)
    }
}
