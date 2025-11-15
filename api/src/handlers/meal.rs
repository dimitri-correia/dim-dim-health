use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json, extract::Path};
use chrono::{DateTime, NaiveDate};
use entities::sea_orm_active_enums::MealTypeEnum;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateMealRequest {
    pub kind: MealTypeEnum,
    pub date: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MealResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: MealTypeEnum,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub created_at: DateTime<chrono::FixedOffset>,
    pub updated_at: DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMealRequest {
    pub kind: Option<MealTypeEnum>,
    pub date: Option<NaiveDate>,
    pub description: Option<Option<String>>,
}

pub async fn create_meal(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<CreateMealRequest>,
) -> Result<Response, Response> {
    info!("Creating meal for user: {}", user.id);

    let meal = state
        .repositories
        .meal_repository
        .create(&user.id, payload.kind, payload.date, payload.description)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok((StatusCode::CREATED, Json(MealResponse {
        id: meal.id,
        user_id: meal.user_id,
        kind: meal.kind,
        date: meal.date,
        description: meal.description,
        created_at: meal.created_at,
        updated_at: meal.updated_at,
    })).into_response())
}

pub async fn get_user_meals(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Response, Response> {
    info!("Fetching meals for user: {}", user.id);

    let meals = state
        .repositories
        .meal_repository
        .find_by_user_id(&user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let response = meals
        .into_iter()
        .map(|m| MealResponse {
            id: m.id,
            user_id: m.user_id,
            kind: m.kind,
            date: m.date,
            description: m.description,
            created_at: m.created_at,
            updated_at: m.updated_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(response).into_response())
}

pub async fn get_meal(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MealResponse>, impl IntoResponse> {
    debug!("Fetching meal {} for user: {}", id, user.id);

    let meal = state
        .repositories
        .meal_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if meal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    Ok(Json(MealResponse {
        id: meal.id,
        user_id: meal.user_id,
        kind: meal.kind,
        date: meal.date,
        description: meal.description,
        created_at: meal.created_at,
        updated_at: meal.updated_at,
    }))
}

pub async fn update_meal(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMealRequest>,
) -> Result<Json<MealResponse>, impl IntoResponse> {
    debug!("Updating meal {} for user: {}", id, user.id);

    // Verify ownership
    let existing = state
        .repositories
        .meal_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if existing.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    let meal = state
        .repositories
        .meal_repository
        .update(&id, payload.kind, payload.date, payload.description)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(MealResponse {
        id: meal.id,
        user_id: meal.user_id,
        kind: meal.kind,
        date: meal.date,
        description: meal.description,
        created_at: meal.created_at,
        updated_at: meal.updated_at,
    }))
}

pub async fn delete_meal(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, impl IntoResponse> {
    debug!("Deleting meal {} for user: {}", id, user.id);

    // Verify ownership
    let existing = state
        .repositories
        .meal_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if existing.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .meal_repository
        .delete(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(json!({"message": "Meal deleted successfully"})))
}
