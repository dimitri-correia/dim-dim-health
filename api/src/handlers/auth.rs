use crate::{
    auth::{
        jwt::generate_token,
        middleware::RequireAuth,
        password::{hash_password, verify_password},
    },
    axummain::state::AppState,
    schemas::auth_schemas::*,
    utils::token_generator::generate_verification_token,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, FixedOffset, Utc};
use log::error;
use serde_json::json;
use tracing::{debug, info};
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserResponse>, impl IntoResponse> {
    info!(
        "Received registration request for: {} [email: {}]",
        payload.user.username, payload.user.email
    );
    if let Err(err) = payload.user.validate() {
        info!("Validation error during registration: {}", err);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    if state
        .repositories
        .user_repository
        .user_already_exists(&payload.user.email, &payload.user.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    {
        info!(
            "Registration attempt with existing email or username: {} [email: {}]",
            payload.user.username, payload.user.email
        );
        return Err(StatusCode::CONFLICT.into_response());
    }

    let password_hash = hash_password(&payload.user.password, None)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    debug!(
        "Creating user: {} [email: {}]",
        payload.user.username, payload.user.email
    );
    let user = state
        .repositories
        .user_repository
        .create(&payload.user.username, &payload.user.email, &password_hash)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to create user because: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    let verification_token = generate_verification_token();
    let offset = FixedOffset::east_opt(0).unwrap();
    let expires_at = Utc::now().with_timezone(&offset) + Duration::hours(2);

    debug!(
        "Generated email verification token for user {}: {}",
        user.id, verification_token
    );
    if let Err(err) = state
        .repositories
        .email_verification_repository
        .create_token(&user.id, &verification_token, &expires_at)
        .await
    {
        error!("Failed to register token because: {err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    debug!(
        "Sending verification email to {}: {}",
        user.email, verification_token
    );
    if let Err(err) = state
        .jobs
        .email_job
        .send_register_email(&user.email, &user.username, &verification_token)
        .await
    {
        error!("Failed to send verification email: {err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

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
    info!("Received login request for email: {}", payload.user.email);
    if let Err(err) = payload.user.validate() {
        info!("Validation error during login: {}", err);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    let user = state
        .repositories
        .user_repository
        .find_by_email(&payload.user.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let user = match user {
        Some(user) => user,
        None => {
            info!(
                "Login attempt with non-existing email: {}",
                payload.user.email
            );
            return Err(StatusCode::UNAUTHORIZED.into_response());
        }
    };

    let password_valid = verify_password(&payload.user.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    if !password_valid {
        info!("Invalid password attempt for email: {}", payload.user.email);
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
    info!("Fetching current user: {}", user.email);
    let token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}

pub async fn verify_email(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Verifying email with params: {:?}", params);

    let token = params.get("token").ok_or(StatusCode::BAD_REQUEST)?;

    let verification_token = state
        .repositories
        .email_verification_repository
        .find_by_token(token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let verification_token = match verification_token {
        Some(token) => token,
        None => {
            info!("Verification token not found: {}", token);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // Should not happen due to query filter, but just in case
    // We delete expired tokens
    if verification_token.is_expired() {
        error!(
            "Verification token expired returned from DB: {} for user {}",
            token, verification_token.user_id
        );
        state
            .repositories
            .email_verification_repository
            .delete_by_token(token)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Err(StatusCode::GONE);
    }

    debug!(
        "Marking user {} email as verified",
        verification_token.user_id
    );
    state
        .repositories
        .email_verification_repository
        .verify_user_email(&verification_token.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    debug!("Deleting verification token: {}", token);
    state
        .repositories
        .email_verification_repository
        .delete_by_token(token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "message": "Email verified successfully!"
    })))

    // TODO: Redirect to frontend verification success page
}
