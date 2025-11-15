use entities::user_weight;
use sea_orm::{
    prelude::Decimal,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
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
        user_id: &Uuid,
        weight_in_kg: Decimal,
        recorded_at: &chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<user_weight::Model, sea_orm::DbErr> {
        let user_weight = user_weight::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id.to_owned()),
            weight_in_kg: Set(weight_in_kg),
            recorded_at: Set(recorded_at.to_owned()),
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

    pub async fn find_latest_by_user_id(
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
        id: &Uuid,
        weight_in_kg: Option<Decimal>,
        recorded_at: Option<&chrono::DateTime<chrono::FixedOffset>>,
    ) -> Result<user_weight::Model, sea_orm::DbErr> {
        if weight_in_kg.is_none() && recorded_at.is_none() {
            return Err(sea_orm::DbErr::Custom(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let mut active = user_weight::ActiveModel {
            id: Set(id.to_owned()),
            ..Default::default()
        };

        if let Some(w) = weight_in_kg {
            active.weight_in_kg = Set(w);
        }

        if let Some(r) = recorded_at {
            active.recorded_at = Set(r.to_owned());
        }

        active.update(&self.db).await
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        let user_weight = user_weight::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await?;

        if let Some(user_weight) = user_weight {
            let user_weight: user_weight::ActiveModel = user_weight.into();
            user_weight.delete(&self.db).await?;
        }

        Ok(())
    }
}
