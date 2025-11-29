use chrono::{DateTime, FixedOffset, NaiveDate};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use entities::sea_orm_active_enums::MuscleEnum;

// ===== Gym Exercise Schemas =====

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGymExerciseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,
    pub primary_muscle: MuscleEnum,
    pub secondary_muscles: Option<Vec<MuscleEnum>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGymExerciseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<Option<String>>,
    pub primary_muscle: Option<MuscleEnum>,
    pub secondary_muscles: Option<Vec<MuscleEnum>>,
}

#[derive(Debug, Serialize)]
pub struct GymExerciseResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub primary_muscle: MuscleEnum,
    pub secondary_muscles: Vec<MuscleEnum>,
    pub added_by: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<entities::gym_exercise::Model> for GymExerciseResponse {
    fn from(exercise: entities::gym_exercise::Model) -> Self {
        Self {
            id: exercise.id,
            name: exercise.name,
            description: exercise.description,
            primary_muscle: exercise.primary_muscle,
            secondary_muscles: exercise.secondary_muscles,
            added_by: exercise.added_by,
            created_at: exercise.created_at,
            updated_at: exercise.updated_at,
        }
    }
}

// ===== Gym Session Schemas =====

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGymSessionRequest {
    #[validate(length(max = 255, message = "Name must be less than 255 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 1000, message = "Notes must be less than 1000 characters"))]
    pub notes: Option<String>,
    pub date: NaiveDate,
    #[validate(range(min = 0, max = 1440, message = "Duration must be between 0 and 1440 minutes"))]
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGymSessionRequest {
    #[validate(length(max = 255, message = "Name must be less than 255 characters"))]
    pub name: Option<Option<String>>,
    #[validate(length(max = 1000, message = "Notes must be less than 1000 characters"))]
    pub notes: Option<Option<String>>,
    pub date: Option<NaiveDate>,
    #[validate(range(min = 0, max = 1440, message = "Duration must be between 0 and 1440 minutes"))]
    pub duration_minutes: Option<Option<i32>>,
}

#[derive(Debug, Serialize)]
pub struct GymSessionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub date: NaiveDate,
    pub duration_minutes: Option<i32>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<entities::gym_session::Model> for GymSessionResponse {
    fn from(session: entities::gym_session::Model) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            name: session.name,
            notes: session.notes,
            date: session.date,
            duration_minutes: session.duration_minutes,
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}

// ===== Gym Set Schemas =====

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGymSetRequest {
    pub exercise_id: Uuid,
    #[validate(range(min = 1, max = 100, message = "Set number must be between 1 and 100"))]
    pub set_number: i32,
    #[validate(range(min = 0, max = 1000, message = "Repetitions must be between 0 and 1000"))]
    pub repetitions: i32,
    pub weight_kg: Decimal,
    #[validate(length(max = 500, message = "Notes must be less than 500 characters"))]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGymSetRequest {
    #[validate(range(min = 1, max = 100, message = "Set number must be between 1 and 100"))]
    pub set_number: Option<i32>,
    #[validate(range(min = 0, max = 1000, message = "Repetitions must be between 0 and 1000"))]
    pub repetitions: Option<i32>,
    pub weight_kg: Option<Decimal>,
    #[validate(length(max = 500, message = "Notes must be less than 500 characters"))]
    pub notes: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct GymSetResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub exercise_id: Uuid,
    pub set_number: i32,
    pub repetitions: i32,
    pub weight_kg: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<entities::gym_set::Model> for GymSetResponse {
    fn from(gym_set: entities::gym_set::Model) -> Self {
        Self {
            id: gym_set.id,
            session_id: gym_set.session_id,
            exercise_id: gym_set.exercise_id,
            set_number: gym_set.set_number,
            repetitions: gym_set.repetitions,
            weight_kg: gym_set.weight_kg,
            notes: gym_set.notes,
            created_at: gym_set.created_at,
            updated_at: gym_set.updated_at,
        }
    }
}
