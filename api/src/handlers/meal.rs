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

    match state
        .repositories
        .meal_repository
        .create(user.id, payload.kind, payload.date, payload.description)
        .await
    {
        Ok(meal) => Ok(Json(MealResponse::from(meal))),
        Err(err) => {
            error!("Failed to create meal: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_meals(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<MealResponse>>, impl IntoResponse> {
    info!("Fetching meals for user: {}", user.id);

    let meals = if let Some(date) = query.date {
        match state
            .repositories
            .meal_repository
            .find_by_user_and_date(&user.id, date)
            .await
        {
            Ok(meals) => meals,
            Err(err) => {
                error!("Failed to fetch meals by date: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    } else {
        match state
            .repositories
            .meal_repository
            .find_by_user_id(&user.id)
            .await
        {
            Ok(meals) => meals,
            Err(err) => {
                error!("Failed to fetch meals: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    };

    let response: Vec<MealResponse> = meals.into_iter().map(MealResponse::from).collect();
    Ok(Json(response))
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
    match state.repositories.meal_repository.find_by_id(&id).await {
        Ok(Some(meal)) => {
            if meal.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch meal: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .meal_repository
        .update(id, payload.kind, payload.date, payload.description)
        .await
    {
        Ok(meal) => Ok(Json(MealResponse::from(meal))),
        Err(err) => {
            error!("Failed to update meal: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_meal(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting meal {} for user: {}", id, user.id);

    // Check if the meal exists and belongs to the user
    match state.repositories.meal_repository.find_by_id(&id).await {
        Ok(Some(meal)) => {
            if meal.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch meal: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state.repositories.meal_repository.delete(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete meal: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
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
    match state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
    {
        Ok(Some(meal)) => {
            if meal.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch meal: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    // Check if the food item exists
    match state
        .repositories
        .food_item_repository
        .find_by_id(&payload.food_item_id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Food item not found"})),
            )
                .into_response());
        }
        Err(err) => {
            error!("Failed to fetch food item: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .meal_item_repository
        .create(meal_id, payload.food_item_id, payload.quantity_in_grams)
        .await
    {
        Ok(meal_item) => Ok(Json(MealItemResponse::from(meal_item))),
        Err(err) => {
            error!("Failed to add meal item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_meal_items(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(meal_id): Path<Uuid>,
) -> Result<Json<Vec<MealItemResponse>>, impl IntoResponse> {
    info!("Fetching items for meal {} for user: {}", meal_id, user.id);

    // Check if the meal exists and belongs to the user
    match state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
    {
        Ok(Some(meal)) => {
            if meal.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch meal: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .meal_item_repository
        .find_by_meal_id(&meal_id)
        .await
    {
        Ok(meal_items) => {
            let response: Vec<MealItemResponse> =
                meal_items.into_iter().map(MealItemResponse::from).collect();
            Ok(Json(response))
        }
        Err(err) => {
            error!("Failed to fetch meal items: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
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
    match state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
    {
        Ok(Some(meal)) => {
            if meal.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch meal: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .meal_item_repository
        .update(item_id, payload.quantity_in_grams)
        .await
    {
        Ok(meal_item) => Ok(Json(MealItemResponse::from(meal_item))),
        Err(err) => {
            error!("Failed to update meal item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
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
    match state
        .repositories
        .meal_repository
        .find_by_id(&meal_id)
        .await
    {
        Ok(Some(meal)) => {
            if meal.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch meal: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .meal_item_repository
        .delete(&item_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete meal item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
