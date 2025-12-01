use crate::{
    auth::{
        middleware::RequireAuth,
        password::{hash_password_async, verify_password_async},
    },
    axummain::state::AppState,
    schemas::settings_schemas::*,
    utils::{get_now_time_paris::now_paris_fixed, token_generator::generate_verification_token},
};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Duration;
use log::error;
use serde_json::json;
use tracing::{debug, info};
use validator::Validate;

pub async fn update_settings(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingsRequest>,
) -> Result<Json<UpdateSettingsResponse>, impl IntoResponse> {
    info!("Received settings update request for user: {}", user.id);

    if let Err(err) = payload.validate() {
        info!("Validation error during settings update: {}", err);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    let mut message = Vec::new();

    // Update username if provided
    if let Some(username) = &payload.username
        && username != &user.username
    {
        // Check if username is already taken
        if let Ok(Some(_)) = state
            .repositories
            .user_repository
            .find_by_username(username)
            .await
        {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({"error": "Username already taken"})),
            )
                .into_response());
        }

        debug!("Updating username for user {}", user.id);
        state
            .repositories
            .user_repository
            .update(&user.id, Some(username), None)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        message.push("Username updated successfully".to_string());
    }

    // Update profile image if provided
    if let Some(profile_image) = payload.profile_image
        && profile_image != user.profile_image
    {
        debug!("Updating profile image for user {}", user.id);
        state
            .repositories
            .user_repository
            .update_profile_image(&user.id, profile_image)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        message.push("Profile image updated successfully".to_string());
    }

    // Update password if provided
    if let Some(ref passwords) = payload.passwords {
        // Verify current password
        let password_valid = verify_password_async(
            passwords.current_password.clone(),
            user.password_hash.clone(),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        if !password_valid {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Current password is incorrect"})),
            )
                .into_response());
        }

        // Hash and update new password
        let new_password_hash = hash_password_async(passwords.new_password.clone(), None)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        debug!("Updating password for user {}", user.id);
        state
            .repositories
            .user_repository
            .update_password(&user.id, &new_password_hash)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        message.push("Password updated successfully".to_string());
    }

    // Handle email change if provided
    if let Some(new_email) = &payload.email
        && new_email != &user.email
    {
        // Check if email is already taken
        if let Ok(Some(_)) = state
            .repositories
            .user_repository
            .find_by_email(new_email)
            .await
        {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({"error": "Email already taken"})),
            )
                .into_response());
        }

        // Create email change verification token
        let verification_token = generate_verification_token();
        let expires_at = now_paris_fixed(Duration::hours(2));

        debug!(
            "Generated email change verification token for user {}: {}",
            user.id, verification_token
        );

        if let Err(err) = state
            .repositories
            .email_verification_repository
            .create_email_change_token(&user.id, &verification_token, &expires_at, new_email)
            .await
        {
            error!("Failed to create email change token because: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }

        debug!(
            "Sending email change verification email to {}: {}",
            new_email, verification_token
        );

        if let Err(err) = state
            .jobs
            .email_job
            .send_email_change_email(new_email, &user.username, &verification_token)
            .await
        {
            error!("Failed to send email change verification email: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }

        message.push(
                "Email change verification sent. Please check your new email address to confirm. Your login email will remain the same until you verify the new email."
                    .to_string(),
            );
    }

    let response_message = if message.is_empty() {
        "No changes made".to_string()
    } else {
        message.join(". ")
    };

    Ok(Json(UpdateSettingsResponse {
        message: response_message,
    }))
}
