use entities::gym_set;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    prelude::Decimal,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct GymSetRepository {
    db: DatabaseConnection,
}

impl GymSetRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        session_id: Uuid,
        exercise_id: Uuid,
        set_number: i32,
        repetitions: i32,
        weight_kg: Decimal,
    ) -> Result<gym_set::Model, sea_orm::DbErr> {
        let gym_set = gym_set::ActiveModel {
            id: NotSet,
            session_id: Set(session_id),
            exercise_id: Set(exercise_id),
            set_number: Set(set_number),
            repetitions: Set(repetitions),
            weight_kg: Set(weight_kg),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let gym_set = gym_set.insert(&self.db).await?;

        Ok(gym_set)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<gym_set::Model>, sea_orm::DbErr> {
        gym_set::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_by_session_id(
        &self,
        session_id: &Uuid,
    ) -> Result<Vec<gym_set::Model>, sea_orm::DbErr> {
        gym_set::Entity::find()
            .filter(gym_set::Column::SessionId.eq(session_id.to_owned()))
            .order_by_asc(gym_set::Column::SetNumber)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        set_number: Option<i32>,
        repetitions: Option<i32>,
        weight_kg: Option<Decimal>,
    ) -> Result<gym_set::Model, sea_orm::DbErr> {
        let mut gym_set: gym_set::ActiveModel = gym_set::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound("Set not found".to_owned()))?
            .into();

        if let Some(set_number) = set_number {
            gym_set.set_number = Set(set_number);
        }
        if let Some(repetitions) = repetitions {
            gym_set.repetitions = Set(repetitions);
        }
        if let Some(weight_kg) = weight_kg {
            gym_set.weight_kg = Set(weight_kg);
        }

        let gym_set = gym_set.update(&self.db).await?;

        Ok(gym_set)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        gym_set::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
