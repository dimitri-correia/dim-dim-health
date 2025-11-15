use entities::{sea_orm_active_enums::UserGroup, user_groups};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct UserGroupsRepository {
    db: DatabaseConnection,
}

impl UserGroupsRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        user_id: &Uuid,
        group: UserGroup,
    ) -> Result<user_groups::Model, sea_orm::DbErr> {
        let user_group = user_groups::ActiveModel {
            user_id: Set(user_id.to_owned()),
            group: Set(group),
            created_at: NotSet,
        };
        let user_group = user_group.insert(&self.db).await?;

        Ok(user_group)
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<user_groups::Model>, sea_orm::DbErr> {
        user_groups::Entity::find()
            .filter(user_groups::Column::UserId.eq(user_id.to_owned()))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_user_id_and_group(
        &self,
        user_id: &Uuid,
        group: UserGroup,
    ) -> Result<(), sea_orm::DbErr> {
        let user_group = user_groups::Entity::find()
            .filter(user_groups::Column::UserId.eq(user_id.to_owned()))
            .filter(user_groups::Column::Group.eq(group))
            .one(&self.db)
            .await?;

        if let Some(user_group) = user_group {
            let user_group: user_groups::ActiveModel = user_group.into();
            user_group.delete(&self.db).await?;
        }

        Ok(())
    }

    pub async fn delete(&self, user_group: user_groups::Model) -> Result<(), sea_orm::DbErr> {
        let user_group: user_groups::ActiveModel = user_group.into();
        user_group.delete(&self.db).await?;
        Ok(())
    }
}
