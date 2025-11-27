use chrono::{DateTime, FixedOffset, NaiveDate};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserWeightRequest {
    pub weight_in_kg: Decimal,
    pub recorded_at: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserWeightRequest {
    pub weight_in_kg: Decimal,
    pub recorded_at: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct UserWeightResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight_in_kg: Decimal,
    pub recorded_at: NaiveDate,
}

impl From<entities::user_weight::Model> for UserWeightResponse {
    fn from(user_weight: entities::user_weight::Model) -> Self {
        Self {
            id: user_weight.id,
            user_id: user_weight.user_id,
            weight_in_kg: user_weight.weight_in_kg,
            recorded_at: user_weight.recorded_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserWeightInfosResponse {
    pub last_3_weights: Vec<UserWeightResponse>,
    pub average_weight: Decimal,
    pub number_of_weight_entries: i64,
    pub average_weight_last_7_days: Decimal,
    pub number_of_weight_entries_last_7_days: i64,
    pub average_weight_last_30_days: Decimal,
    pub number_of_weight_entries_last_30_days: i64,
    pub max_weight: Decimal,
    pub max_weight_date: DateTime<FixedOffset>,
    pub min_weight: Decimal,
    pub min_weight_date: DateTime<FixedOffset>,
}
