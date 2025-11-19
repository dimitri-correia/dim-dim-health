use crate::{
    auth::middleware::RequireAuth, axummain::state::AppState, schemas::user_weight_schemas::*,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

pub async fn create_user_weight(
    State(state): State<AppState>,
    RequireAuth(user): RequireAuth,
    Json(payload): Json<CreateUserWeightRequest>,
) -> Result<Json<UserWeightResponse>, impl IntoResponse> {
    info!("Creating weight entry for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    match state
        .repositories
        .user_weight_repository
        .create(user.id, payload.weight_in_kg, payload.recorded_at)
        .await
    {
        Ok(user_weight) => Ok(Json(UserWeightResponse::from(user_weight))),
        Err(err) => {
            error!("Failed to create user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_user_weights(
    State(state): State<AppState>,
    RequireAuth(user): RequireAuth,
) -> Result<Json<Vec<UserWeightResponse>>, impl IntoResponse> {
    info!("Fetching weight entries for user: {}", user.id);

    match state
        .repositories
        .user_weight_repository
        .find_by_user_id(&user.id)
        .await
    {
        Ok(weights) => {
            let response: Vec<UserWeightResponse> =
                weights.into_iter().map(UserWeightResponse::from).collect();
            Ok(Json(response))
        }
        Err(err) => {
            error!("Failed to fetch user weights: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_user_weight_by_id(
    State(state): State<AppState>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<Uuid>,
) -> Result<Json<UserWeightResponse>, impl IntoResponse> {
    info!("Fetching weight entry {} for user: {}", id, user.id);

    match state
        .repositories
        .user_weight_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(weight)) => {
            // Ensure the weight entry belongs to the authenticated user
            if weight.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
            Ok(Json(UserWeightResponse::from(weight)))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn update_user_weight(
    State(state): State<AppState>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserWeightRequest>,
) -> Result<Json<UserWeightResponse>, impl IntoResponse> {
    info!("Updating weight entry {} for user: {}", id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // First check if the weight entry exists and belongs to the user
    match state
        .repositories
        .user_weight_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(weight)) => {
            if weight.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch user weight: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .user_weight_repository
        .update(id, payload.weight_in_kg, payload.recorded_at)
        .await
    {
        Ok(user_weight) => Ok(Json(UserWeightResponse::from(user_weight))),
        Err(err) => {
            error!("Failed to update user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_user_weight(
    State(state): State<AppState>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting weight entry {} for user: {}", id, user.id);

    // First check if the weight entry exists and belongs to the user
    match state
        .repositories
        .user_weight_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(weight)) => {
            if weight.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch user weight: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state.repositories.user_weight_repository.delete(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
