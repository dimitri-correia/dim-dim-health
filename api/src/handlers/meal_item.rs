use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, extract::Path};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateMealItemRequest {
    pub meal_id: Uuid,
    pub food_item_id: Uuid,
    pub quantity_in_grams: i32,
}

#[derive(Debug, Serialize)]
pub struct MealItemResponse {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub food_item_id: Uuid,
    pub quantity_in_grams: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMealItemRequest {
    pub food_item_id: Option<Uuid>,
    pub quantity_in_grams: Option<i32>,
}

pub async fn create_meal_item(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<CreateMealItemRequest>,
) -> Result<Json<MealItemResponse>, impl IntoResponse> {
    info!("Creating meal item for meal: {}", payload.meal_id);

    // Verify the meal belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&payload.meal_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    // Verify the food item exists
    state
        .repositories
        .food_item_repository
        .find_by_id(&payload.food_item_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    let meal_item = state
        .repositories
        .meal_item_repository
        .create(&payload.meal_id, &payload.food_item_id, payload.quantity_in_grams)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(MealItemResponse {
        id: meal_item.id,
        meal_id: meal_item.meal_id,
        food_item_id: meal_item.food_item_id,
        quantity_in_grams: meal_item.quantity_in_grams,
    }))
}

pub async fn get_meal_items(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(meal_id): Path<Uuid>,
) -> Result<Json<Vec<MealItemResponse>>, impl IntoResponse> {
    info!("Fetching meal items for meal: {}", meal_id);

    // Verify the meal belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    let meal_items = state
        .repositories
        .meal_item_repository
        .find_by_meal_id(&meal_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let response = meal_items
        .into_iter()
        .map(|mi| MealItemResponse {
            id: mi.id,
            meal_id: mi.meal_id,
            food_item_id: mi.food_item_id,
            quantity_in_grams: mi.quantity_in_grams,
        })
        .collect();

    Ok(Json(response))
}

pub async fn update_meal_item(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMealItemRequest>,
) -> Result<Json<MealItemResponse>, impl IntoResponse> {
    debug!("Updating meal item {}", id);

    // Verify ownership through meal
    let existing = state
        .repositories
        .meal_item_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&existing.meal_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    // If updating food_item_id, verify it exists
    if let Some(food_item_id) = &payload.food_item_id {
        state
            .repositories
            .food_item_repository
            .find_by_id(food_item_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
            .ok_or(StatusCode::NOT_FOUND.into_response())?;
    }

    let meal_item = state
        .repositories
        .meal_item_repository
        .update(&id, payload.food_item_id.as_ref(), payload.quantity_in_grams)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(MealItemResponse {
        id: meal_item.id,
        meal_id: meal_item.meal_id,
        food_item_id: meal_item.food_item_id,
        quantity_in_grams: meal_item.quantity_in_grams,
    }))
}

pub async fn delete_meal_item(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, impl IntoResponse> {
    debug!("Deleting meal item {}", id);

    // Verify ownership through meal
    let existing = state
        .repositories
        .meal_item_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&existing.meal_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_item_repository
        .delete(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(json!({"message": "Meal item deleted successfully"})))
}
