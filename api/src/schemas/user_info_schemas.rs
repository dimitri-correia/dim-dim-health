use chrono::{DateTime, FixedOffset, NaiveDate};
use entities::sea_orm_active_enums::GenderEnum;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserInfoRequest {
    pub birth_date: NaiveDate,
    #[validate(range(min = 50, max = 300, message = "Height must be between 50 and 300 cm"))]
    pub height_in_cm: i32,
    pub gender: GenderEnum,
    pub activity_level: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserInfoRequest {
    pub birth_date: Option<NaiveDate>,
    #[validate(range(min = 50, max = 300, message = "Height must be between 50 and 300 cm"))]
    pub height_in_cm: Option<i32>,
    pub gender: Option<GenderEnum>,
    pub activity_level: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub user_id: uuid::Uuid,
    pub birth_date: NaiveDate,
    pub height_in_cm: i32,
    pub gender: GenderEnum,
    pub activity_level: Decimal,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<entities::user_additional_infos::Model> for UserInfoResponse {
    fn from(info: entities::user_additional_infos::Model) -> Self {
        Self {
            user_id: info.user_id,
            birth_date: info.birth_date,
            height_in_cm: info.height_in_cm,
            gender: info.gender,
            activity_level: info.activity_level,
            updated_at: info.updated_at,
        }
    }
}
