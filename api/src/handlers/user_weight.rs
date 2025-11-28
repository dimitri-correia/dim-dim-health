use crate::{
    auth::middleware::RequireVerifiedAuth, axummain::state::AppState,
    schemas::user_weight_schemas::*, weight::weight_infos::user_weight_infos,
};
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde_json::json;
use tracing::{error, info, warn};
use uuid::Uuid;
use validator::Validate;

/// Checks if a database error is a unique constraint violation
fn is_unique_constraint_violation(err: &DbErr) -> bool {
    let error_str = err.to_string().to_lowercase();
    error_str.contains("unique constraint") || error_str.contains("duplicate key")
}

/// Helper function to check if a user has permission to view another user's data
async fn check_view_permission(
    state: &AppState,
    current_user_id: &Uuid,
    target_user_id: &Uuid,
) -> Result<(), Response<Body>> {
    let can_view = state
        .services
        .authorization
        .can_view_user_data(current_user_id, target_user_id)
        .await
        .map_err(|err| {
            error!("Failed to check view permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if !can_view {
        return Err(StatusCode::FORBIDDEN.into_response());
    }

    Ok(())
}

pub async fn create_user_weight(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
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
            if is_unique_constraint_violation(&err) {
                warn!(
                    "Duplicate weight entry attempt for user {} on date {}",
                    user.id, payload.recorded_at
                );
                return Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "A weight entry already exists for this date"})),
                )
                    .into_response());
            }
            error!("Failed to create user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_user_weights(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
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

pub async fn get_user_last_weight(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
) -> Result<Json<UserWeightResponse>, impl IntoResponse> {
    info!("Fetching last weight entry for user: {}", user.id);

    match state
        .repositories
        .user_weight_repository
        .find_last_by_user_id(&user.id)
        .await
    {
        Ok(Some(weight)) => Ok(Json(UserWeightResponse::from(weight))),
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch last user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_user_weight_infos(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
) -> Result<Json<Option<UserWeightInfosResponse>>, impl IntoResponse> {
    info!("Fetching weight infos for user: {}", user.id);

    match state
        .repositories
        .user_weight_repository
        .find_by_user_id(&user.id)
        .await
    {
        Ok(weights) => Ok(Json(user_weight_infos(weights))),
        Err(err) => {
            error!("Failed to fetch user weights: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn update_user_weight(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
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
            if is_unique_constraint_violation(&err) {
                warn!(
                    "Duplicate weight entry attempt for user {} on date {}",
                    user.id, payload.recorded_at
                );
                return Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "A weight entry already exists for this date"})),
                )
                    .into_response());
            }
            error!("Failed to update user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_user_weight(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
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

/// Get weights for another user if the current user has permission to view them
pub async fn get_other_user_weights(
    State(state): State<AppState>,
    RequireVerifiedAuth(current_user): RequireVerifiedAuth,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<UserWeightResponse>>, impl IntoResponse> {
    info!(
        "User {} fetching weight entries for user: {}",
        current_user.id, user_id
    );

    check_view_permission(&state, &current_user.id, &user_id).await?;

    match state
        .repositories
        .user_weight_repository
        .find_by_user_id(&user_id)
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

/// Get weight infos for another user if the current user has permission to view them
pub async fn get_other_user_weight_infos(
    State(state): State<AppState>,
    RequireVerifiedAuth(current_user): RequireVerifiedAuth,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Option<UserWeightInfosResponse>>, impl IntoResponse> {
    info!(
        "User {} fetching weight infos for user: {}",
        current_user.id, user_id
    );

    check_view_permission(&state, &current_user.id, &user_id).await?;

    match state
        .repositories
        .user_weight_repository
        .find_by_user_id(&user_id)
        .await
    {
        Ok(weights) => Ok(Json(user_weight_infos(weights))),
        Err(err) => {
            error!("Failed to fetch user weights: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

/// Get last weight for another user if the current user has permission to view them
pub async fn get_other_user_last_weight(
    State(state): State<AppState>,
    RequireVerifiedAuth(current_user): RequireVerifiedAuth,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserWeightResponse>, impl IntoResponse> {
    info!(
        "User {} fetching last weight entry for user: {}",
        current_user.id, user_id
    );

    check_view_permission(&state, &current_user.id, &user_id).await?;

    match state
        .repositories
        .user_weight_repository
        .find_last_by_user_id(&user_id)
        .await
    {
        Ok(Some(weight)) => Ok(Json(UserWeightResponse::from(weight))),
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch last user weight: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
