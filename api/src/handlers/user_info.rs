use crate::{
    auth::middleware::RequireVerifiedAuth, axummain::state::AppState,
    schemas::user_info_schemas::*,
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use tracing::{error, info};
use validator::Validate;

pub async fn get_user_info(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
) -> Result<Json<UserInfoResponse>, impl IntoResponse> {
    info!("Fetching additional info for user: {}", user.id);

    match state
        .repositories
        .user_info_repository
        .find_by_user_id(&user.id)
        .await
    {
        Ok(Some(info)) => Ok(Json(UserInfoResponse::from(info))),
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch user info: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn create_user_info(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Json(payload): Json<CreateUserInfoRequest>,
) -> Result<Json<UserInfoResponse>, impl IntoResponse> {
    info!("Creating additional info for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if info already exists
    match state
        .repositories
        .user_info_repository
        .find_by_user_id(&user.id)
        .await
    {
        Ok(Some(_)) => {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({"error": "User info already exists. Use PUT to update."})),
            )
                .into_response());
        }
        Ok(None) => {}
        Err(err) => {
            error!("Failed to check existing user info: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .user_info_repository
        .create(
            &user.id,
            &payload.birth_date,
            payload.height_in_cm,
            payload.gender,
            payload.activity_level,
        )
        .await
    {
        Ok(info) => Ok(Json(UserInfoResponse::from(info))),
        Err(err) => {
            error!("Failed to create user info: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn update_user_info(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Json(payload): Json<UpdateUserInfoRequest>,
) -> Result<Json<UserInfoResponse>, impl IntoResponse> {
    info!("Updating additional info for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check at least one field is provided
    if payload.birth_date.is_none()
        && payload.height_in_cm.is_none()
        && payload.gender.is_none()
        && payload.activity_level.is_none()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "At least one field must be provided for update"})),
        )
            .into_response());
    }

    // Check if info exists
    match state
        .repositories
        .user_info_repository
        .find_by_user_id(&user.id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User info not found. Use POST to create."})),
            )
                .into_response());
        }
        Err(err) => {
            error!("Failed to check existing user info: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .user_info_repository
        .update(
            &user.id,
            payload.birth_date.as_ref(),
            payload.height_in_cm,
            payload.gender,
            payload.activity_level,
        )
        .await
    {
        Ok(info) => Ok(Json(UserInfoResponse::from(info))),
        Err(err) => {
            error!("Failed to update user info: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
