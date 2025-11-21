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

    match state
        .repositories
        .food_item_repository
        .create(payload, user.id)
        .await
    {
        Ok(food_item) => Ok(Json(FoodItemResponse::from(food_item))),
        Err(err) => {
            error!("Failed to create food item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_food_items(
    State(state): State<AppState>,
    RequireVerifiedAuth(_user): RequireVerifiedAuth,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<FoodItemResponse>>, impl IntoResponse> {
    info!("Fetching food items");

    let food_items = if let Some(scan_code) = query.scan_code {
        match state
            .repositories
            .food_item_repository
            .find_by_scan_code(&scan_code)
            .await
        {
            Ok(Some(item)) => vec![item],
            Ok(None) => vec![],
            Err(err) => {
                error!("Failed to fetch food items by scan code: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    } else if let Some(name) = query.name {
        match state
            .repositories
            .food_item_repository
            .find_by_name(&name)
            .await
        {
            Ok(items) => items,
            Err(err) => {
                error!("Failed to fetch food items by name: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    } else {
        match state.repositories.food_item_repository.find_all().await {
            Ok(items) => items,
            Err(err) => {
                error!("Failed to fetch all food items: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    };

    let response: Vec<FoodItemResponse> =
        food_items.into_iter().map(FoodItemResponse::from).collect();
    Ok(Json(response))
}

pub async fn get_food_item_by_id(
    State(state): State<AppState>,
    RequireVerifiedAuth(_user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<Json<FoodItemResponse>, impl IntoResponse> {
    info!("Fetching food item {}", id);

    match state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(food_item)) => Ok(Json(FoodItemResponse::from(food_item))),
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch food item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
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
    match state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(food_item)) => {
            if food_item.added_by != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch food item: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .food_item_repository
        .update(id, payload)
        .await
    {
        Ok(food_item) => Ok(Json(FoodItemResponse::from(food_item))),
        Err(err) => {
            error!("Failed to update food item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_food_item(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting food item {} for user: {}", id, user.id);

    // Check if the food item exists and belongs to the user
    match state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(food_item)) => {
            if food_item.added_by != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch food item: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state.repositories.food_item_repository.delete(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete food item: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
