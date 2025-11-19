use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserWeightRequest {
    pub weight_in_kg: Decimal,
    pub recorded_at: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserWeightRequest {
    pub weight_in_kg: Decimal,
    pub recorded_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize)]
pub struct UserWeightResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight_in_kg: Decimal,
    pub recorded_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
}

impl From<entities::user_weight::Model> for UserWeightResponse {
    fn from(user_weight: entities::user_weight::Model) -> Self {
        Self {
            id: user_weight.id,
            user_id: user_weight.user_id,
            weight_in_kg: user_weight.weight_in_kg,
            recorded_at: user_weight.recorded_at,
            created_at: user_weight.created_at,
        }
    }
}
