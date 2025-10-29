use crate::{
    auth::{
        jwt::generate_token,
        middleware::RequireAuth,
        password::{hash_password, verify_password},
    },
    axummain::state::AppState,
    schemas::auth_schemas::*,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserResponse>, impl IntoResponse> {
    if let Err(err) = payload.user.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    if state
        .user_repository
        .user_already_exists(&payload.user.email, &payload.user.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    {
        return Err(StatusCode::CONFLICT.into_response());
    }

    let password_hash = hash_password(&payload.user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let user = state
        .user_repository
        .create(&payload.user.username, &payload.user.email, &password_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<UserResponse>, impl IntoResponse> {
    if let Err(err) = payload.user.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    let user = state
        .user_repository
        .find_by_email(&payload.user.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        .ok_or(StatusCode::UNAUTHORIZED.into_response())?;

    let password_valid = verify_password(&payload.user.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    if !password_valid {
        return Err(StatusCode::UNAUTHORIZED.into_response());
    }

    let token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}

pub async fn current_user(
    State(state): State<AppState>,
    RequireAuth(user): RequireAuth,
) -> Result<Json<UserResponse>, StatusCode> {
    let token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}
