use entities::user_watch_permissions;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct UserWatchPermissionRepository {
    db: DatabaseConnection,
}

impl UserWatchPermissionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        user_watched_id: &Uuid,
        user_watching_id: &Uuid,
    ) -> Result<user_watch_permissions::Model, sea_orm::DbErr> {
        let user_watch_permissions = user_watch_permissions::ActiveModel {
            user_watched_id: Set(*user_watched_id),
            user_watching_id: Set(*user_watching_id),
            created_at: NotSet,
        };
        let user_watch_permissions = user_watch_permissions.insert(&self.db).await?;

        Ok(user_watch_permissions)
    }

    pub async fn find_all_watched(
        &self,
        user_watched_id: &Uuid,
    ) -> Result<Vec<users::Model>, sea_orm::DbErr> {
        let res = user_watch_permissions::Entity::find()
            .filter(user_watch_permissions::Column::UserWatchedId.eq(*user_watched_id))
            .find_also_related(users::Entity)
            .all(&self.db)
            .await?;

        Ok(res.into_iter().filter_map(|(_, user)| user).collect())
    }

    pub async fn find_all_watching(
        &self,
        user_watching_id: &Uuid,
    ) -> Result<Vec<users::Model>, sea_orm::DbErr> {
        let res = user_watch_permissions::Entity::find()
            .filter(user_watch_permissions::Column::UserWatchingId.eq(*user_watching_id))
            .find_also_related(users::Entity)
            .all(&self.db)
            .await?;

        Ok(res.into_iter().filter_map(|(_, user)| user).collect())
    }

    pub async fn find_by_user_ids(
        &self,
        user_watched_id: &Uuid,
        user_watching_id: &Uuid,
    ) -> Result<Option<user_watch_permissions::Model>, sea_orm::DbErr> {
        user_watch_permissions::Entity::find()
            .filter(user_watch_permissions::Column::UserWatchedId.eq(*user_watched_id))
            .filter(user_watch_permissions::Column::UserWatchingId.eq(*user_watching_id))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_user_ids(
        &self,
        user_watched_id: &Uuid,
        user_watching_id: &Uuid,
    ) -> Result<(), sea_orm::DbErr> {
        let user_watch_permissions = user_watch_permissions::Entity::find()
            .filter(user_watch_permissions::Column::UserWatchedId.eq(*user_watched_id))
            .filter(user_watch_permissions::Column::UserWatchingId.eq(*user_watching_id))
            .one(&self.db)
            .await?;

        if let Some(user_watch_permissions) = user_watch_permissions {
            let user_watch_permissions: user_watch_permissions::ActiveModel =
                user_watch_permissions.into();
            user_watch_permissions.delete(&self.db).await?;
        }

        Ok(())
    }
}
