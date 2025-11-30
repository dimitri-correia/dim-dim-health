use chrono::NaiveDate;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use entities::sea_orm_active_enums::MuscleEnum;

fn validate_weight_kg(weight: &Decimal) -> Result<(), ValidationError> {
    if *weight < Decimal::ZERO {
        return Err(ValidationError::new("weight_kg must be non-negative"));
    }
    if *weight > Decimal::new(999999, 2) {
        return Err(ValidationError::new("weight_kg must be less than 10000 kg"));
    }
    Ok(())
}

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
    pub primary_muscles: Vec<MuscleEnum>,
    pub secondary_muscles: Vec<MuscleEnum>,
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
    pub primary_muscles: Option<Vec<MuscleEnum>>,
    pub secondary_muscles: Option<Vec<MuscleEnum>>,
}

#[derive(Debug, Serialize)]
pub struct GymExerciseResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub primary_muscles: Vec<MuscleEnum>,
    pub secondary_muscles: Vec<MuscleEnum>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGymSessionRequest {
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGymSessionRequest {
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct GymSessionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
}

impl From<entities::gym_session::Model> for GymSessionResponse {
    fn from(session: entities::gym_session::Model) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            date: session.date,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGymSetRequest {
    pub exercise_id: Uuid,
    #[validate(range(min = 1, max = 100, message = "Set number must be between 1 and 100"))]
    pub set_number: i32,
    #[validate(range(
        min = 0,
        max = 1000,
        message = "Repetitions must be between 0 and 1000"
    ))]
    pub repetitions: i32,
    #[validate(custom(function = "validate_weight_kg"))]
    pub weight_kg: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGymSetRequest {
    #[validate(range(min = 1, max = 100, message = "Set number must be between 1 and 100"))]
    pub set_number: Option<i32>,
    #[validate(range(
        min = 0,
        max = 1000,
        message = "Repetitions must be between 0 and 1000"
    ))]
    pub repetitions: Option<i32>,
    #[validate(custom(function = "validate_weight_kg"))]
    pub weight_kg: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct GymSetResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub exercise_id: Uuid,
    pub set_number: i32,
    pub repetitions: i32,
    pub weight_kg: Decimal,
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
        }
    }
}
