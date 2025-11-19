use chrono::{DateTime, FixedOffset, NaiveDate};
use entities::sea_orm_active_enums::MealTypeEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMealRequest {
    pub kind: MealTypeEnum,
    pub date: NaiveDate,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMealRequest {
    pub kind: Option<MealTypeEnum>,
    pub date: Option<NaiveDate>,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct MealResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: MealTypeEnum,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<entities::meal::Model> for MealResponse {
    fn from(meal: entities::meal::Model) -> Self {
        Self {
            id: meal.id,
            user_id: meal.user_id,
            kind: meal.kind,
            date: meal.date,
            description: meal.description,
            created_at: meal.created_at,
            updated_at: meal.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddMealItemRequest {
    pub food_item_id: Uuid,
    #[validate(range(min = 1, max = 100000, message = "Quantity must be between 1 and 100000 grams"))]
    pub quantity_in_grams: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMealItemRequest {
    #[validate(range(min = 1, max = 100000, message = "Quantity must be between 1 and 100000 grams"))]
    pub quantity_in_grams: i32,
}

#[derive(Debug, Serialize)]
pub struct MealItemResponse {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub food_item_id: Uuid,
    pub quantity_in_grams: i32,
}

impl From<entities::meal_item::Model> for MealItemResponse {
    fn from(meal_item: entities::meal_item::Model) -> Self {
        Self {
            id: meal_item.id,
            meal_id: meal_item.meal_id,
            food_item_id: meal_item.food_item_id,
            quantity_in_grams: meal_item.quantity_in_grams,
        }
    }
}
