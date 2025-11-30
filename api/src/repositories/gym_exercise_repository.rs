use entities::{
    exercise_muscle, gym_exercise,
    sea_orm_active_enums::{MuscleEnum, MuscleRoleEnum},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};
use uuid::Uuid;

use crate::schemas::gym_schemas::GymExerciseResponse;

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
        primary_muscles: Vec<MuscleEnum>,
        secondary_muscles: Vec<MuscleEnum>,
        added_by: Uuid,
    ) -> Result<GymExerciseResponse, sea_orm::DbErr> {
        // TODO - fix and use only one call or least possible
        let exercise = gym_exercise::ActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
            added_by: Set(added_by),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let exercise = exercise.insert(&self.db).await?;

        for pr in primary_muscles {
            let primary = exercise_muscle::ActiveModel {
                id: NotSet,
                exercise_id: sea_orm::Set(sea_orm::Unchanged(exercise.id)),
                muscle: pr,
                role: sea_orm::Set(sea_orm::Unchanged(MuscleRoleEnum::Primary)),
                created_at: NotSet,
                updated_at: NotSet,
            };
            let primary = primary.insert(&self.db).await?;
        }
        for pr in secondary_muscles {
            let secondary = exercise_muscle::ActiveModel {
                id: NotSet,
                exercise_id: sea_orm::Set(sea_orm::Unchanged(exercise.id)),
                muscle: pr,
                role: sea_orm::Set(sea_orm::Unchanged(MuscleRoleEnum::Secondary)),
                created_at: NotSet,
                updated_at: NotSet,
            };
            let secondary = secondary.insert(&self.db).await?;
        }

        let gym_exercice_response = GymExerciseResponse {
            id: exercise.id,
            name: exercise.name,
            description: exercise.description,
            primary_muscles,
            secondary_muscles,
        };

        Ok(gym_exercice_response)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<gym_exercise::Model>, sea_orm::DbErr> {
        todo!()
    }

    pub async fn find_all(&self) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        todo!()
    }

    pub async fn find_by_primary_muscle(
        &self,
        muscle: MuscleEnum,
    ) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        todo!()
    }

    pub async fn find_by_secondary_muscle(
        &self,
        muscle: MuscleEnum,
    ) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        todo!()
    }

    pub async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<gym_exercise::Model>, sea_orm::DbErr> {
        todo!()
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<Option<String>>,
        primary_muscle: Option<MuscleEnum>,
        secondary_muscles: Option<Vec<MuscleEnum>>,
    ) -> Result<gym_exercise::Model, sea_orm::DbErr> {
        todo!()
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        todo!()
    }
}
