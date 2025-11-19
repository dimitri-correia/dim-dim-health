use entities::user_weight;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    prelude::Decimal,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserWeightRepository {
    db: DatabaseConnection,
}

impl UserWeightRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        weight_in_kg: Decimal,
        recorded_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<user_weight::Model, sea_orm::DbErr> {
        let user_weight = user_weight::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            weight_in_kg: Set(weight_in_kg),
            recorded_at: Set(recorded_at),
            created_at: NotSet,
        };
        let user_weight = user_weight.insert(&self.db).await?;

        Ok(user_weight)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<user_weight::Model>, sea_orm::DbErr> {
        user_weight::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<user_weight::Model>, sea_orm::DbErr> {
        user_weight::Entity::find()
            .filter(user_weight::Column::UserId.eq(user_id.to_owned()))
            .order_by_desc(user_weight::Column::RecordedAt)
            .all(&self.db)
            .await
    }

    pub async fn find_last_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<user_weight::Model>, sea_orm::DbErr> {
        user_weight::Entity::find()
            .filter(user_weight::Column::UserId.eq(user_id.to_owned()))
            .order_by_desc(user_weight::Column::RecordedAt)
            .one(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        weight_in_kg: Decimal,
        recorded_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<user_weight::Model, sea_orm::DbErr> {
        let user_weight = user_weight::ActiveModel {
            id: Set(id),
            weight_in_kg: Set(weight_in_kg),
            recorded_at: Set(recorded_at),
            ..Default::default()
        };
        let user_weight = user_weight.update(&self.db).await?;

        Ok(user_weight)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        user_weight::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
