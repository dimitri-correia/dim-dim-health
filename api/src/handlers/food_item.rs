use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json, extract::Path, extract::Query};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateFoodItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub scan_code: Option<String>,
    pub calories_per100g: i32,
    pub protein_per100g: i32,
    pub carbs_per100g: i32,
    pub fat_per100g: i32,
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
    pub added_at: DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFoodItemRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub scan_code: Option<Option<String>>,
    pub calories_per100g: Option<i32>,
    pub protein_per100g: Option<i32>,
    pub carbs_per100g: Option<i32>,
    pub fat_per100g: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQueryParams {
    pub name: Option<String>,
}

pub async fn create_food_item(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<CreateFoodItemRequest>,
) -> Result<Response, Response> {
    info!("Creating food item: {} by user: {}", payload.name, user.id);

    let food_item = state
        .repositories
        .food_item_repository
        .create(
            &payload.name,
            payload.description,
            payload.scan_code,
            payload.calories_per100g,
            payload.protein_per100g,
            payload.carbs_per100g,
            payload.fat_per100g,
            &user.id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok((StatusCode::CREATED, Json(FoodItemResponse {
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
    })).into_response())
}

pub async fn get_food_items(
    RequireAuth(_user): RequireAuth,
    State(state): State<AppState>,
    Query(params): Query<SearchQueryParams>,
) -> Result<Response, Response> {
    info!("Fetching food items");

    let food_items = if let Some(name) = params.name {
        info!("Searching food items by name: {}", name);
        state
            .repositories
            .food_item_repository
            .search_by_name(&name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    } else {
        state
            .repositories
            .food_item_repository
            .find_all()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    };

    let response = food_items
        .into_iter()
        .map(|f| FoodItemResponse {
            id: f.id,
            name: f.name,
            description: f.description,
            scan_code: f.scan_code,
            calories_per100g: f.calories_per100g,
            protein_per100g: f.protein_per100g,
            carbs_per100g: f.carbs_per100g,
            fat_per100g: f.fat_per100g,
            added_by: f.added_by,
            added_at: f.added_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(response).into_response())
}

pub async fn get_food_item(
    RequireAuth(_user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Response, Response> {
    debug!("Fetching food item: {}", id);

    let food_item = state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    Ok(Json(FoodItemResponse {
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
    }).into_response())
}

pub async fn update_food_item(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFoodItemRequest>,
) -> Result<Json<FoodItemResponse>, impl IntoResponse> {
    debug!("Updating food item {} for user: {}", id, user.id);

    // Verify ownership
    let existing = state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if existing.added_by != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    let food_item = state
        .repositories
        .food_item_repository
        .update(
            &id,
            payload.name.as_deref(),
            payload.description,
            payload.scan_code,
            payload.calories_per100g,
            payload.protein_per100g,
            payload.carbs_per100g,
            payload.fat_per100g,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(FoodItemResponse {
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
    }))
}

pub async fn delete_food_item(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, impl IntoResponse> {
    debug!("Deleting food item {} for user: {}", id, user.id);

    // Verify ownership
    let existing = state
        .repositories
        .food_item_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if existing.added_by != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .food_item_repository
        .delete(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(json!({"message": "Food item deleted successfully"})))
}
