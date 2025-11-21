use entities::{
    users,
    users_partial::{UserAuthModel, UserBasicModel, UserEmailVerificationModel, UserPublicModel},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
        is_guest: bool,
    ) -> Result<users::Model, sea_orm::DbErr> {
        let user = users::ActiveModel {
            id: NotSet,
            username: Set(username.to_owned()),
            email: Set(email.to_owned()),
            password_hash: Set(password_hash.to_owned()),
            created_at: NotSet,
            updated_at: NotSet,
            email_verified: if is_guest { Set(true) } else { NotSet },
        };
        let user = user.insert(&self.db).await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<users::Model>, sea_orm::DbErr> {
        users::Entity::find_by_id(id.to_owned()).one(&self.db).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, sea_orm::DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .one(&self.db)
            .await
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<users::Model>, sea_orm::DbErr> {
        users::Entity::find()
            .filter(users::Column::Username.eq(username.to_owned()))
            .one(&self.db)
            .await
    }

    pub async fn user_already_exists(
        &self,
        email: &str,
        username: &str,
    ) -> Result<bool, sea_orm::DbErr> {
        let count = users::Entity::find()
            .filter(
                users::Column::Email
                    .eq(email)
                    .or(users::Column::Username.eq(username)),
            )
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    pub async fn update(
        &self,
        id: &Uuid,
        username: Option<&str>,
        email: Option<&str>,
    ) -> Result<users::Model, sea_orm::DbErr> {
        if username.is_none() && email.is_none() {
            return Err(sea_orm::DbErr::Custom(
                "At least one field (username or email) must be provided for update".to_string(),
            ));
        }

        let mut active = users::ActiveModel {
            id: Set(id.to_owned()),
            ..Default::default()
        };

        if let Some(u) = username {
            active.username = Set(u.to_owned());
        }
        if let Some(e) = email {
            active.email = Set(e.to_owned());
        }

        active.update(&self.db).await
    }

    pub async fn update_password(
        &self,
        id: &Uuid,
        password_hash: &str,
    ) -> Result<users::Model, sea_orm::DbErr> {
        let active = users::ActiveModel {
            id: Set(id.to_owned()),
            password_hash: Set(password_hash.to_owned()),
            ..Default::default()
        };

        active.update(&self.db).await
    }

    // Partial model queries for optimized database access

    /// Find user by email for authentication - returns only fields needed for login
    pub async fn find_by_email_for_auth(
        &self,
        email: &str,
    ) -> Result<Option<UserAuthModel>, sea_orm::DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .select_only()
            .column(users::Column::Id)
            .column(users::Column::Username)
            .column(users::Column::Email)
            .column(users::Column::PasswordHash)
            .column(users::Column::EmailVerified)
            .into_model::<UserAuthModel>()
            .one(&self.db)
            .await
    }

    /// Find user by ID with only public information
    pub async fn find_by_id_public(
        &self,
        id: &Uuid,
    ) -> Result<Option<UserPublicModel>, sea_orm::DbErr> {
        users::Entity::find_by_id(id.to_owned())
            .select_only()
            .column(users::Column::Id)
            .column(users::Column::Username)
            .column(users::Column::Email)
            .column(users::Column::EmailVerified)
            .into_model::<UserPublicModel>()
            .one(&self.db)
            .await
    }

    /// Find user by email with only basic information
    pub async fn find_by_email_basic(
        &self,
        email: &str,
    ) -> Result<Option<UserBasicModel>, sea_orm::DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .select_only()
            .column(users::Column::Id)
            .column(users::Column::Username)
            .column(users::Column::Email)
            .into_model::<UserBasicModel>()
            .one(&self.db)
            .await
    }

    /// Check email verification status for a user
    pub async fn find_email_verification_status(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<UserEmailVerificationModel>, sea_orm::DbErr> {
        users::Entity::find_by_id(user_id.to_owned())
            .select_only()
            .column(users::Column::Id)
            .column(users::Column::Email)
            .column(users::Column::EmailVerified)
            .into_model::<UserEmailVerificationModel>()
            .one(&self.db)
            .await
    }
}
