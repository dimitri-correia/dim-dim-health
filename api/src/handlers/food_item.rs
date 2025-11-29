use crate::{
    auth::middleware::RequireVerifiedAuth, axummain::state::AppState, schemas::food_item_schemas::*,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub name: Option<String>,
    pub scan_code: Option<String>,
}

pub async fn create_food_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Json(payload): Json<CreateFoodItemRequest>,
) -> Result<Json<FoodItemResponse>, impl IntoResponse> {
    info!("Creating food item for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    state
        .repositories
        .food_item_repository
        .create(payload, user.id)
        .await
        .map(|food_item| Json(FoodItemResponse::from(food_item)))
        .map_err(|err| {
            error!("Failed to create food item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn get_food_items(
    State(state): State<AppState>,
    RequireVerifiedAuth(_user): RequireVerifiedAuth,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<FoodItemResponse>>, impl IntoResponse> {
    info!("Fetching food items");

    let food_items_result = if let Some(scan_code) = query.scan_code {
        state
            .repositories
            .food_item_repository
            .find_by_scan_code(&scan_code)
            .await
            .map(|opt| opt.into_iter().collect())
    } else if let Some(name) = query.name {
        state
            .repositories
            .food_item_repository
            .find_by_name(&name)
            .await
    } else {
        state.repositories.food_item_repository.find_all().await
    };

    food_items_result
        .map(|items| Json(items.into_iter().map(FoodItemResponse::from).collect()))
        .map_err(|err| {
            error!("Failed to fetch food items: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn update_food_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFoodItemRequest>,
) -> Result<Json<FoodItemResponse>, impl IntoResponse> {
    info!("Updating food item {} for user: {}", id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the food item exists and belongs to the user
    let food_item = state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
        .map_err(|err| {
            error!("Failed to fetch food item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if food_item.added_by != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .food_item_repository
        .update(id, payload)
        .await
        .map(|food_item| Json(FoodItemResponse::from(food_item)))
        .map_err(|err| {
            error!("Failed to update food item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn delete_food_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting food item {} for user: {}", id, user.id);

    // Check if the food item exists and belongs to the user
    let food_item = state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
        .map_err(|err| {
            error!("Failed to fetch food item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    if food_item.added_by != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .food_item_repository
        .delete(&id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("Failed to delete food item: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}
