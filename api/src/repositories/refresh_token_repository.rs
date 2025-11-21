use chrono::Duration;
use entities::{refresh_token, token_partial::RefreshTokenValidationModel};
use migration::Expr;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Value,
};

use uuid::Uuid;

use crate::utils::get_now_time_paris::now_paris_fixed;

#[derive(Clone)]
pub struct RefreshTokenRepository {
    db: DatabaseConnection,
}

impl RefreshTokenRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_token(
        &self,
        user_id: &Uuid,
        token: &str,
    ) -> Result<refresh_token::Model, sea_orm::DbErr> {
        let refresh_token = refresh_token::ActiveModel {
            id: NotSet,
            user_id: Set(user_id.to_owned()),
            token: Set(token.to_owned()),
            created_at: NotSet,
            expires_at: NotSet,
            used_at: NotSet,
        };
        let refresh_token = refresh_token.insert(&self.db).await?;

        Ok(refresh_token)
    }

    pub async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<refresh_token::Model>, sea_orm::DbErr> {
        refresh_token::Entity::find()
            .filter(refresh_token::Column::Token.eq(token))
            .one(&self.db)
            .await
    }

    pub async fn mark_token_as_used(&self, token: &str) -> Result<(), sea_orm::DbErr> {
        let now = now_paris_fixed(Duration::zero());
        refresh_token::Entity::update_many()
            .col_expr(
                refresh_token::Column::UsedAt,
                Expr::value(Value::ChronoDateTimeWithTimeZone(Some(now))),
            )
            .filter(refresh_token::Column::Token.eq(token))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn delete_by_token(&self, token: &str) -> Result<bool, sea_orm::DbErr> {
        let token = refresh_token::Entity::find()
            .filter(refresh_token::Column::Token.eq(token))
            .one(&self.db)
            .await?;

        if let Some(token) = token {
            let active_model: refresh_token::ActiveModel = token.into();
            active_model.delete(&self.db).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn delete_all_user_tokens(&self, user_id: &Uuid) -> Result<bool, sea_orm::DbErr> {
        refresh_token::Entity::delete_many()
            .filter(refresh_token::Column::UserId.eq(*user_id))
            .exec(&self.db)
            .await?;

        Ok(true)
    }

    // Partial model queries for optimized database access

    /// Find token for validation - returns only fields needed for validation
    pub async fn find_by_token_for_validation(
        &self,
        token: &str,
    ) -> Result<Option<RefreshTokenValidationModel>, sea_orm::DbErr> {
        refresh_token::Entity::find()
            .filter(refresh_token::Column::Token.eq(token))
            .select_only()
            .column(refresh_token::Column::UserId)
            .column(refresh_token::Column::ExpiresAt)
            .column(refresh_token::Column::UsedAt)
            .into_model::<RefreshTokenValidationModel>()
            .one(&self.db)
            .await
    }
}
