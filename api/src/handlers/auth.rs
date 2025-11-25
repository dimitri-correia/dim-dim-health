use crate::utils::guest_name_generator::GUEST_EMAIL_DOMAIN;
use crate::{
    auth::{
        jwt::generate_token,
        middleware::RequireAuth,
        password::{hash_password, verify_password},
        refresh_token::generate_refresh_token,
    },
    axummain::state::AppState,
    schemas::{
        auth_schemas::*,
        password_reset_schemas::{
            ForgotPasswordRequest, ForgotPasswordResponse, ResetPasswordRequest,
        },
        token_schemas::{LogoutRequest, LogoutResponse, RefreshTokenRequest, RefreshTokenResponse},
    },
    utils::{get_now_time_paris::now_paris_fixed, token_generator::generate_verification_token},
};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Duration;
use log::error;
use serde_json::json;
use tracing::{debug, info};
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<LoginResponse>, impl IntoResponse> {
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

    common_register_logic(
        state,
        payload.user.username,
        payload.user.email,
        password_hash,
        false,
    )
    .await
    .map_err(|e| e.into_response())
}

pub async fn register_guest(
    State(state): State<AppState>,
) -> Result<Json<LoginResponse>, impl IntoResponse> {
    let username = loop {
        let candidate = crate::utils::guest_name_generator::generate_guest_name();
        if state
            .repositories
            .user_repository
            .ensure_username_not_taken(&candidate)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
        {
            break candidate;
        }
    };

    let email = format!("{username}{GUEST_EMAIL_DOMAIN}");
    let password_hash = hash_password("password", Some(4))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    common_register_logic(state, username, email, password_hash, true)
        .await
        .map_err(|e| e.into_response())
}

async fn common_register_logic(
    state: AppState,
    username: String,
    email: String,
    password_hash: String,
    is_guest: bool,
) -> Result<Json<LoginResponse>, impl IntoResponse> {
    debug!("Creating user: {} [email: {}]", username, email);
    let user = state
        .repositories
        .user_repository
        .create(&username, &email, &password_hash, is_guest)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to create user because: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    if !is_guest {
        let verification_token = generate_verification_token();
        // If updated, need to be changed in the mail too
        let expires_at = now_paris_fixed(Duration::hours(2));

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
    }

    let access_token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
    let refresh_token = generate_refresh_token();

    debug!("Creating refresh token for user {}", &user.id);
    state
        .repositories
        .refresh_token_repository
        .create_token(&user.id, &refresh_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let user_id = user.id;
    let user_data = UserData::from_user(user);
    let response = LoginResponse {
        user: user_data,
        access_token,
        refresh_token,
    };

    if is_guest {
        state
            .repositories
            .user_group_repository
            .create(
                &user_id,
                entities::sea_orm_active_enums::UserGroup::GuestGroup,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
    }

    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<LoginResponse>, impl IntoResponse> {
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

    let access_token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
    let refresh_token = generate_refresh_token();

    debug!("Creating refresh token for user {}", user.id);
    state
        .repositories
        .refresh_token_repository
        .create_token(&user.id, &refresh_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let user_data = UserData::from_user(user);
    let response = LoginResponse {
        user: user_data,
        access_token,
        refresh_token,
    };

    Ok(Json(response))
}

pub async fn current_user(
    RequireAuth(user): RequireAuth,
) -> Result<Json<UserResponse>, StatusCode> {
    info!("Fetching current user: {}", user.email);
    let user_data = UserData::from_user(user);
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

    // Check if this is an email change verification or initial verification
    if let Some(pending_email) = &verification_token.pending_email {
        debug!(
            "Updating user {} email to {}",
            verification_token.user_id, pending_email
        );
        state
            .repositories
            .email_verification_repository
            .update_user_email(&verification_token.user_id, pending_email)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
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
    }

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

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ForgotPasswordResponse>, impl IntoResponse> {
    info!(
        "Received forgot password request for email: {}",
        payload.email
    );

    let ok_response = Ok(Json(ForgotPasswordResponse {
        message: "If that email exists, a password reset link has been sent.".to_string(),
    }));

    if let Err(err) = payload.validate() {
        info!("Validation error during forgot password: {}", err);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    let user = state
        .repositories
        .user_repository
        .find_by_email(&payload.email)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to query user by email because: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    // Always return success even if email doesn't exist
    // This prevents attackers from discovering which emails are registered
    // Would be good to take some time here to mitigate timing attacks
    let user = match user {
        Some(user) => user,
        None => {
            info!(
                "Forgot password request for non-existing email: {}",
                payload.email
            );
            return ok_response;
        }
    };

    let reset_token = generate_verification_token();
    // If updated, need to be changed in the mail too
    let expires_at = now_paris_fixed(Duration::hours(1));

    debug!(
        "Generated password reset token for user {}: {}",
        user.id, reset_token
    );
    if let Err(err) = state
        .repositories
        .password_reset_repository
        .create_token(&user.id, &reset_token, &expires_at)
        .await
    {
        error!("Failed to create password reset token because: {err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };

    debug!(
        "Sending password reset email to {}: {}",
        user.email, reset_token
    );
    if let Err(err) = state
        .jobs
        .email_job
        .send_password_reset_email(&user.email, &user.username, &reset_token)
        .await
    {
        error!("Failed to send password reset email: {err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };

    ok_response
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<LoginResponse>, impl IntoResponse> {
    info!(
        "Received reset password request for token: {}",
        payload.token
    );
    if let Err(err) = payload.validate() {
        info!("Validation error during reset password: {}", err);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    let reset_token = state
        .repositories
        .password_reset_repository
        .find_by_token(&payload.token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
    let reset_token = match reset_token {
        Some(token) => token,
        None => {
            info!("Password reset token not found: {}", payload.token);
            return Err(StatusCode::NOT_FOUND.into_response());
        }
    };

    // Should not happen due to query filter, but just in case
    // We delete expired tokens
    if reset_token.is_expired() {
        error!(
            "Password reset token expired returned from DB: {} for user {}",
            payload.token, reset_token.user_id
        );
        state
            .repositories
            .password_reset_repository
            .delete_by_token(&payload.token)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        return Err(StatusCode::GONE.into_response());
    }

    let new_password_hash = hash_password(&payload.new_password, None)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    debug!("Updating password for user {}", reset_token.user_id);
    let user = state
        .repositories
        .user_repository
        .update_password(&reset_token.user_id, &new_password_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    // Delete ALL reset tokens for this user (invalidate any other pending requests)
    state
        .repositories
        .password_reset_repository
        .delete_all_user_tokens(&reset_token.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    // Generate tokens for auto-login
    let access_token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let refresh_token = generate_refresh_token();
    let refresh_token_hash = hash_password(&refresh_token, Some(4))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    state
        .repositories
        .refresh_token_repository
        .create_token(&user.id, &refresh_token_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(Json(LoginResponse {
        user: UserData::from_user(user),
        access_token,
        refresh_token,
    }))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, StatusCode> {
    info!(
        "Received refresh token request for token: {}",
        payload.refresh_token
    );
    let refresh_token = state
        .repositories
        .refresh_token_repository
        .find_by_token(&payload.refresh_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if refresh_token.is_expired() {
        info!("Refresh token expired: {}", payload.refresh_token);
        state
            .repositories
            .refresh_token_repository
            .delete_by_token(&payload.refresh_token)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Err(StatusCode::UNAUTHORIZED);
    }

    if refresh_token.used_at.is_some() {
        // SECURITY BREACH DETECTED!
        // Someone is trying to use an old token
        // This means the token was likely stolen (or error in the client logic)
        error!(
            "[NOT SUPPOSED TO HAPPEN] Refresh token already used: {}",
            payload.refresh_token
        );
        state
            .repositories
            .refresh_token_repository
            .delete_all_user_tokens(&refresh_token.user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Err(StatusCode::UNAUTHORIZED);
    }

    state
        .repositories
        .refresh_token_repository
        .mark_token_as_used(&payload.refresh_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_refresh_token = generate_refresh_token();

    state
        .repositories
        .refresh_token_repository
        .create_token(&refresh_token.user_id, &new_refresh_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_token = generate_token(&refresh_token.user_id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RefreshTokenResponse {
        access_token,
        refresh_token: new_refresh_token,
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, StatusCode> {
    state
        .repositories
        .refresh_token_repository
        .delete_by_token(&payload.refresh_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LogoutResponse {
        message: "Logged out successfully".to_string(),
    }))
}
