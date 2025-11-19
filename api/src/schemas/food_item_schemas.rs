use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFoodItemRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: String,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,
    pub scan_code: Option<String>,
    #[validate(range(min = 0, max = 10000, message = "Calories must be between 0 and 10000"))]
    pub calories_per100g: i32,
    #[validate(range(min = 0, max = 1000, message = "Protein must be between 0 and 1000"))]
    pub protein_per100g: i32,
    #[validate(range(min = 0, max = 1000, message = "Carbs must be between 0 and 1000"))]
    pub carbs_per100g: i32,
    #[validate(range(min = 0, max = 1000, message = "Fat must be between 0 and 1000"))]
    pub fat_per100g: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFoodItemRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<Option<String>>,
    pub scan_code: Option<Option<String>>,
    #[validate(range(min = 0, max = 10000, message = "Calories must be between 0 and 10000"))]
    pub calories_per100g: Option<i32>,
    #[validate(range(min = 0, max = 1000, message = "Protein must be between 0 and 1000"))]
    pub protein_per100g: Option<i32>,
    #[validate(range(min = 0, max = 1000, message = "Carbs must be between 0 and 1000"))]
    pub carbs_per100g: Option<i32>,
    #[validate(range(min = 0, max = 1000, message = "Fat must be between 0 and 1000"))]
    pub fat_per100g: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct FoodItemResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scan_code: Option<String>,
    pub calories_per100g: i32,
    pub protein_per100g: i32,
    pub carbs_per100g: i32,
    pub fat_per100g: i32,
    pub added_by: Uuid,
    pub added_at: DateTime<FixedOffset>,
}

impl From<entities::food_item::Model> for FoodItemResponse {
    fn from(food_item: entities::food_item::Model) -> Self {
        Self {
            id: food_item.id,
            name: food_item.name,
            description: food_item.description,
            scan_code: food_item.scan_code,
            calories_per100g: food_item.calories_per100g,
            protein_per100g: food_item.protein_per100g,
            carbs_per100g: food_item.carbs_per100g,
            fat_per100g: food_item.fat_per100g,
            added_by: food_item.added_by,
            added_at: food_item.added_at,
        }
    }
}
