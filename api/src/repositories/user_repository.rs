use entities::{sea_orm_active_enums::UserGroup, user_groups, users};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, ExprTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
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
        users::Entity::find_by_id(*id).one(&self.db).await
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

    pub async fn ensure_username_not_taken(&self, username: &str) -> Result<bool, sea_orm::DbErr> {
        let count = users::Entity::find()
            .filter(users::Column::Username.eq(username.to_owned()))
            .count(&self.db)
            .await?;

        Ok(count == 0)
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
            id: Set(*id),
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
            id: Set(*id),
            password_hash: Set(password_hash.to_owned()),
            ..Default::default()
        };

        active.update(&self.db).await
    }

    pub async fn search_by_username(
        &self,
        query: &str,
    ) -> Result<Vec<users::Model>, sea_orm::DbErr> {
        users::Entity::find()
            .join(JoinType::LeftJoin, users::Relation::UserGroups.def())
            .filter(
                Condition::all()
                    .add(users::Column::Username.contains(query))
                    .add(users::Column::EmailVerified.eq(true))
                    .add(
                        user_groups::Column::Group
                            .ne(UserGroup::GuestGroup)
                            .or(user_groups::Column::Group.is_null()),
                    ),
            )
            .limit(20)
            .all(&self.db)
            .await
    }
}
