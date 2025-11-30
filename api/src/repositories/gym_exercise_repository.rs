use entities::{gym_exercise, sea_orm_active_enums::MuscleEnum};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct GymExerciseRepository {
    db: DatabaseConnection,
}

impl GymExerciseRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        primary_muscle: MuscleEnum,
        secondary_muscles: Vec<MuscleEnum>,
        added_by: Uuid,
    ) -> Result<gym_exercise::Model, sea_orm::DbErr> {
        let exercise = gym_exercise::ActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
            primary_muscle: Set(primary_muscle),
            secondary_muscles: Set(secondary_muscles),
            added_by: Set(added_by),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let exercise = exercise.insert(&self.db).await?;

        Ok(exercise)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<gym_exercise::Model>, sea_orm::DbErr> {
        gym_exercise::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        gym_exercise::Entity::find()
            .order_by_asc(gym_exercise::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn find_by_muscle(
        &self,
        muscle: MuscleEnum,
    ) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        gym_exercise::Entity::find()
            .filter(gym_exercise::Column::PrimaryMuscle.eq(muscle))
            .order_by_asc(gym_exercise::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        gym_exercise::Entity::find()
            .filter(gym_exercise::Column::Name.contains(name))
            .order_by_asc(gym_exercise::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<Option<String>>,
        primary_muscle: Option<MuscleEnum>,
        secondary_muscles: Option<Vec<MuscleEnum>>,
    ) -> Result<gym_exercise::Model, sea_orm::DbErr> {
        let mut exercise: gym_exercise::ActiveModel = gym_exercise::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Exercise not found".to_owned(),
            ))?
            .into();

        if let Some(name) = name {
            exercise.name = Set(name);
        }
        if let Some(description) = description {
            exercise.description = Set(description);
        }
        if let Some(primary_muscle) = primary_muscle {
            exercise.primary_muscle = Set(primary_muscle);
        }
        if let Some(secondary_muscles) = secondary_muscles {
            exercise.secondary_muscles = Set(secondary_muscles);
        }

        let exercise = exercise.update(&self.db).await?;

        Ok(exercise)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        gym_exercise::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
