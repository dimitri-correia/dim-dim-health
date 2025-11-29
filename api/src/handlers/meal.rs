use crate::{
    auth::middleware::RequireVerifiedAuth, axummain::state::AppState, schemas::meal_schemas::*,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub date: Option<NaiveDate>,
}

pub async fn create_meal(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Json(payload): Json<CreateMealRequest>,
) -> Result<Json<MealResponse>, impl IntoResponse> {
    info!("Creating meal for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    state
        .repositories
        .meal_repository
        .create(user.id, payload.kind, payload.date, payload.description)
        .await
        .map(|meal| Json(MealResponse::from(meal)))
        .map_err(|err| {
            error!("Failed to create meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn get_meals(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<MealResponse>>, impl IntoResponse> {
    info!("Fetching meals for user: {}", user.id);

    let meals_result = if let Some(date) = query.date {
        state
            .repositories
            .meal_repository
            .find_by_user_and_date(&user.id, date)
            .await
    } else {
        state
            .repositories
            .meal_repository
            .find_by_user_id(&user.id)
            .await
    };

    meals_result
        .map(|meals| Json(meals.into_iter().map(MealResponse::from).collect()))
        .map_err(|err| {
            error!("Failed to fetch meals: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn update_meal(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMealRequest>,
) -> Result<Json<MealResponse>, impl IntoResponse> {
    info!("Updating meal {} for user: {}", id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the meal exists and belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&id)
        .await
        .map_err(|err| {
            error!("Failed to fetch meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_repository
        .update(id, payload.kind, payload.date, payload.description)
        .await
        .map(|meal| Json(MealResponse::from(meal)))
        .map_err(|err| {
            error!("Failed to update meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn delete_meal(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting meal {} for user: {}", id, user.id);

    // Check if the meal exists and belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&id)
        .await
        .map_err(|err| {
            error!("Failed to fetch meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_repository
        .delete(&id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("Failed to delete meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

// Meal item handlers
pub async fn add_meal_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(meal_id): Path<Uuid>,
    Json(payload): Json<AddMealItemRequest>,
) -> Result<Json<MealItemResponse>, impl IntoResponse> {
    info!("Adding item to meal {} for user: {}", meal_id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the meal exists and belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
        .map_err(|err| {
            error!("Failed to fetch meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    // Check if the food item exists
    if state
        .repositories
        .food_item_repository
        .find_by_id(&payload.food_item_id)
        .await
        .map_err(|err| {
            error!("Failed to fetch food item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .is_none()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Food item not found"})),
        )
            .into_response());
    }

    state
        .repositories
        .meal_item_repository
        .create(meal_id, payload.food_item_id, payload.quantity_in_grams)
        .await
        .map(|meal_item| Json(MealItemResponse::from(meal_item)))
        .map_err(|err| {
            error!("Failed to add meal item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn get_meal_items(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(meal_id): Path<Uuid>,
) -> Result<Json<Vec<MealItemResponse>>, impl IntoResponse> {
    info!("Fetching items for meal {} for user: {}", meal_id, user.id);

    // Check if the meal exists and belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
        .map_err(|err| {
            error!("Failed to fetch meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_item_repository
        .find_by_meal_id(&meal_id)
        .await
        .map(|meal_items| Json(meal_items.into_iter().map(MealItemResponse::from).collect()))
        .map_err(|err| {
            error!("Failed to fetch meal items: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn update_meal_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path((meal_id, item_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMealItemRequest>,
) -> Result<Json<MealItemResponse>, impl IntoResponse> {
    info!(
        "Updating item {} in meal {} for user: {}",
        item_id, meal_id, user.id
    );

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the meal exists and belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
        .map_err(|err| {
            error!("Failed to fetch meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_item_repository
        .update(item_id, payload.quantity_in_grams)
        .await
        .map(|meal_item| Json(MealItemResponse::from(meal_item)))
        .map_err(|err| {
            error!("Failed to update meal item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn delete_meal_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path((meal_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, impl IntoResponse> {
    info!(
        "Deleting item {} from meal {} for user: {}",
        item_id, meal_id, user.id
    );

    // Check if the meal exists and belongs to the user
    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
        .map_err(|err| {
            error!("Failed to fetch meal: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_item_repository
        .delete(&item_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("Failed to delete meal item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}
