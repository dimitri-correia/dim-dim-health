use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json, extract::Path};
use chrono::DateTime;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserWeightRequest {
    pub weight_in_kg: Decimal,
    pub recorded_at: Option<DateTime<chrono::FixedOffset>>,
}

#[derive(Debug, Serialize)]
pub struct UserWeightResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight_in_kg: Decimal,
    pub recorded_at: DateTime<chrono::FixedOffset>,
    pub created_at: DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserWeightRequest {
    pub weight_in_kg: Option<Decimal>,
    pub recorded_at: Option<DateTime<chrono::FixedOffset>>,
}

pub async fn create_user_weight(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<CreateUserWeightRequest>,
) -> Result<Response, Response> {
    info!("Creating weight entry for user: {}", user.id);

    let recorded_at = payload.recorded_at.unwrap_or_else(|| chrono::Utc::now().into());

    let weight = state
        .repositories
        .user_weight_repository
        .create(&user.id, payload.weight_in_kg, &recorded_at)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok((StatusCode::CREATED, Json(UserWeightResponse {
        id: weight.id,
        user_id: weight.user_id,
        weight_in_kg: weight.weight_in_kg,
        recorded_at: weight.recorded_at,
        created_at: weight.created_at,
    })).into_response())
}

pub async fn get_user_weights(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Response, Response> {
    info!("Fetching weight entries for user: {}", user.id);

    let weights = state
        .repositories
        .user_weight_repository
        .find_by_user_id(&user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let response = weights
        .into_iter()
        .map(|w| UserWeightResponse {
            id: w.id,
            user_id: w.user_id,
            weight_in_kg: w.weight_in_kg,
            recorded_at: w.recorded_at,
            created_at: w.created_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(response).into_response())
}

pub async fn get_latest_user_weight(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Response, Response> {
    info!("Fetching latest weight entry for user: {}", user.id);

    let weight = state
        .repositories
        .user_weight_repository
        .find_latest_by_user_id(&user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    Ok(Json(UserWeightResponse {
        id: weight.id,
        user_id: weight.user_id,
        weight_in_kg: weight.weight_in_kg,
        recorded_at: weight.recorded_at,
        created_at: weight.created_at,
    }).into_response())
}

pub async fn update_user_weight(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserWeightRequest>,
) -> Result<Json<UserWeightResponse>, impl IntoResponse> {
    debug!("Updating weight entry {} for user: {}", id, user.id);

    // Verify ownership
    let existing = state
        .repositories
        .user_weight_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if existing.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    let weight = state
        .repositories
        .user_weight_repository
        .update(&id, payload.weight_in_kg, payload.recorded_at.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(UserWeightResponse {
        id: weight.id,
        user_id: weight.user_id,
        weight_in_kg: weight.weight_in_kg,
        recorded_at: weight.recorded_at,
        created_at: weight.created_at,
    }))
}

pub async fn delete_user_weight(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, impl IntoResponse> {
    debug!("Deleting weight entry {} for user: {}", id, user.id);

    // Verify ownership
    let existing = state
        .repositories
        .user_weight_repository
        .find_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::NOT_FOUND.into_response())?;

    if existing.user_id != user.id {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    state
        .repositories
        .user_weight_repository
        .delete(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(json!({"message": "Weight entry deleted successfully"})))
}
