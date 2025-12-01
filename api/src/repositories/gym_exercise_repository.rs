use entities::{
    exercise_muscle, gym_exercise,
    sea_orm_active_enums::{MuscleEnum, MuscleRoleEnum},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
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
        let exercise = gym_exercise::ActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
            added_by: Set(added_by),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let exercise = exercise.insert(&self.db).await?;

        for muscle in &primary_muscles {
            let primary = exercise_muscle::ActiveModel {
                id: NotSet,
                exercise_id: Set(exercise.id),
                muscle: Set(muscle.clone()),
                role: Set(MuscleRoleEnum::Primary),
                created_at: NotSet,
                updated_at: NotSet,
            };
            primary.insert(&self.db).await?;
        }
        for muscle in &secondary_muscles {
            let secondary = exercise_muscle::ActiveModel {
                id: NotSet,
                exercise_id: Set(exercise.id),
                muscle: Set(muscle.clone()),
                role: Set(MuscleRoleEnum::Secondary),
                created_at: NotSet,
                updated_at: NotSet,
            };
            secondary.insert(&self.db).await?;
        }

        let gym_exercise_response = GymExerciseResponse {
            id: exercise.id,
            name: exercise.name,
            description: exercise.description,
            primary_muscles,
            secondary_muscles,
        };

        Ok(gym_exercise_response)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<gym_exercise::Model>, sea_orm::DbErr> {
        gym_exercise::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_by_id_with_muscles(
        &self,
        id: &Uuid,
    ) -> Result<Option<GymExerciseResponse>, sea_orm::DbErr> {
        let exercise = gym_exercise::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await?;

        match exercise {
            Some(exercise) => {
                let muscles = exercise_muscle::Entity::find()
                    .filter(exercise_muscle::Column::ExerciseId.eq(exercise.id))
                    .all(&self.db)
                    .await?;

                let primary_muscles: Vec<MuscleEnum> = muscles
                    .iter()
                    .filter(|m| m.role == MuscleRoleEnum::Primary)
                    .map(|m| m.muscle.clone())
                    .collect();
                let secondary_muscles: Vec<MuscleEnum> = muscles
                    .iter()
                    .filter(|m| m.role == MuscleRoleEnum::Secondary)
                    .map(|m| m.muscle.clone())
                    .collect();

                Ok(Some(GymExerciseResponse {
                    id: exercise.id,
                    name: exercise.name,
                    description: exercise.description,
                    primary_muscles,
                    secondary_muscles,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<GymExerciseResponse>, sea_orm::DbErr> {
        let exercises = gym_exercise::Entity::find().all(&self.db).await?;

        let mut responses = Vec::new();
        for exercise in exercises {
            let muscles = exercise_muscle::Entity::find()
                .filter(exercise_muscle::Column::ExerciseId.eq(exercise.id))
                .all(&self.db)
                .await?;

            let primary_muscles: Vec<MuscleEnum> = muscles
                .iter()
                .filter(|m| m.role == MuscleRoleEnum::Primary)
                .map(|m| m.muscle.clone())
                .collect();
            let secondary_muscles: Vec<MuscleEnum> = muscles
                .iter()
                .filter(|m| m.role == MuscleRoleEnum::Secondary)
                .map(|m| m.muscle.clone())
                .collect();

            responses.push(GymExerciseResponse {
                id: exercise.id,
                name: exercise.name,
                description: exercise.description,
                primary_muscles,
                secondary_muscles,
            });
        }

        Ok(responses)
    }

    pub async fn find_by_muscle(
        &self,
        muscle: MuscleEnum,
    ) -> Result<Vec<GymExerciseResponse>, sea_orm::DbErr> {
        // Find all exercise_muscle entries with this muscle
        let exercise_muscles = exercise_muscle::Entity::find()
            .filter(exercise_muscle::Column::Muscle.eq(muscle))
            .all(&self.db)
            .await?;

        let exercise_ids: Vec<Uuid> = exercise_muscles.iter().map(|em| em.exercise_id).collect();

        // Get unique exercise ids
        let mut unique_ids: Vec<Uuid> = exercise_ids.clone();
        unique_ids.sort();
        unique_ids.dedup();

        let mut responses = Vec::new();
        for exercise_id in unique_ids {
            if let Some(response) = self.find_by_id_with_muscles(&exercise_id).await? {
                responses.push(response);
            }
        }

        Ok(responses)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Vec<GymExerciseResponse>, sea_orm::DbErr> {
        let exercises = gym_exercise::Entity::find()
            .filter(gym_exercise::Column::Name.contains(name))
            .all(&self.db)
            .await?;

        let mut responses = Vec::new();
        for exercise in exercises {
            let muscles = exercise_muscle::Entity::find()
                .filter(exercise_muscle::Column::ExerciseId.eq(exercise.id))
                .all(&self.db)
                .await?;

            let primary_muscles: Vec<MuscleEnum> = muscles
                .iter()
                .filter(|m| m.role == MuscleRoleEnum::Primary)
                .map(|m| m.muscle.clone())
                .collect();
            let secondary_muscles: Vec<MuscleEnum> = muscles
                .iter()
                .filter(|m| m.role == MuscleRoleEnum::Secondary)
                .map(|m| m.muscle.clone())
                .collect();

            responses.push(GymExerciseResponse {
                id: exercise.id,
                name: exercise.name,
                description: exercise.description,
                primary_muscles,
                secondary_muscles,
            });
        }

        Ok(responses)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<Option<String>>,
        primary_muscles: Option<Vec<MuscleEnum>>,
        secondary_muscles: Option<Vec<MuscleEnum>>,
    ) -> Result<GymExerciseResponse, sea_orm::DbErr> {
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

        let exercise = exercise.update(&self.db).await?;

        // Update muscles if provided
        if primary_muscles.is_some() || secondary_muscles.is_some() {
            // Delete existing muscles
            exercise_muscle::Entity::delete_many()
                .filter(exercise_muscle::Column::ExerciseId.eq(id))
                .exec(&self.db)
                .await?;

            // Insert new primary muscles
            if let Some(ref muscles) = primary_muscles {
                for muscle in muscles {
                    let new_muscle = exercise_muscle::ActiveModel {
                        id: NotSet,
                        exercise_id: Set(id),
                        muscle: Set(muscle.clone()),
                        role: Set(MuscleRoleEnum::Primary),
                        created_at: NotSet,
                        updated_at: NotSet,
                    };
                    new_muscle.insert(&self.db).await?;
                }
            }

            // Insert new secondary muscles
            if let Some(ref muscles) = secondary_muscles {
                for muscle in muscles {
                    let new_muscle = exercise_muscle::ActiveModel {
                        id: NotSet,
                        exercise_id: Set(id),
                        muscle: Set(muscle.clone()),
                        role: Set(MuscleRoleEnum::Secondary),
                        created_at: NotSet,
                        updated_at: NotSet,
                    };
                    new_muscle.insert(&self.db).await?;
                }
            }
        }

        // Fetch the updated muscles
        let muscles = exercise_muscle::Entity::find()
            .filter(exercise_muscle::Column::ExerciseId.eq(id))
            .all(&self.db)
            .await?;

        let final_primary_muscles: Vec<MuscleEnum> = muscles
            .iter()
            .filter(|m| m.role == MuscleRoleEnum::Primary)
            .map(|m| m.muscle.clone())
            .collect();
        let final_secondary_muscles: Vec<MuscleEnum> = muscles
            .iter()
            .filter(|m| m.role == MuscleRoleEnum::Secondary)
            .map(|m| m.muscle.clone())
            .collect();

        Ok(GymExerciseResponse {
            id: exercise.id,
            name: exercise.name,
            description: exercise.description,
            primary_muscles: final_primary_muscles,
            secondary_muscles: final_secondary_muscles,
        })
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        // Muscles will be deleted via cascade
        gym_exercise::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
