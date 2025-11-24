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
            ResetPasswordResponse,
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
) -> Result<Json<ResetPasswordResponse>, impl IntoResponse> {
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
    state
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

    Ok(Json(ResetPasswordResponse {
        message: "Password has been reset successfully. You can now login with your new password."
            .to_string(),
    }))
}

pub async fn reset_password_page(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token").unwrap_or(&String::new()).clone();
    
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reset Password - DimDim Health</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }}
        .container {{
            background: white;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            max-width: 450px;
            width: 100%;
            padding: 40px;
        }}
        .header {{
            text-align: center;
            margin-bottom: 32px;
        }}
        .icon {{
            font-size: 64px;
            margin-bottom: 16px;
        }}
        h1 {{
            color: #d97706;
            font-size: 28px;
            margin-bottom: 8px;
        }}
        .subtitle {{
            color: #6b7280;
            font-size: 14px;
        }}
        .form-group {{
            margin-bottom: 20px;
        }}
        label {{
            display: block;
            color: #374151;
            font-weight: 500;
            margin-bottom: 8px;
            font-size: 14px;
        }}
        input {{
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e5e7eb;
            border-radius: 8px;
            font-size: 16px;
            transition: border-color 0.2s;
        }}
        input:focus {{
            outline: none;
            border-color: #d97706;
        }}
        .error {{
            color: #dc2626;
            font-size: 14px;
            margin-top: 8px;
            display: none;
        }}
        .error.show {{
            display: block;
        }}
        .submit-btn {{
            width: 100%;
            padding: 14px;
            background: #d97706;
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: background 0.2s;
            margin-top: 24px;
        }}
        .submit-btn:hover {{
            background: #b45309;
        }}
        .submit-btn:disabled {{
            background: #9ca3af;
            cursor: not-allowed;
        }}
        .success {{
            display: none;
            text-align: center;
        }}
        .success.show {{
            display: block;
        }}
        .success-icon {{
            width: 80px;
            height: 80px;
            background: #10b981;
            border-radius: 50%;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 40px;
            color: white;
            margin-bottom: 24px;
        }}
        .success h2 {{
            color: #d97706;
            margin-bottom: 16px;
        }}
        .success p {{
            color: #6b7280;
            margin-bottom: 24px;
        }}
        .login-link {{
            display: inline-block;
            color: #d97706;
            text-decoration: none;
            font-weight: 600;
        }}
        .login-link:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="icon">🔐</div>
            <h1>Reset Password</h1>
            <p class="subtitle">Enter your new password below</p>
        </div>

        <form id="resetForm">
            <div class="form-group">
                <label for="password">New Password</label>
                <input type="password" id="password" name="password" 
                       placeholder="Enter new password" required minlength="8">
                <div class="error" id="passwordError">Password must be at least 8 characters</div>
            </div>

            <div class="form-group">
                <label for="confirmPassword">Confirm Password</label>
                <input type="password" id="confirmPassword" name="confirmPassword" 
                       placeholder="Confirm new password" required>
                <div class="error" id="confirmError">Passwords do not match</div>
            </div>

            <div class="error" id="apiError"></div>

            <button type="submit" class="submit-btn" id="submitBtn">Reset Password</button>
        </form>

        <div class="success" id="successView">
            <div class="success-icon">✓</div>
            <h2>Password Reset!</h2>
            <p>Your password has been reset successfully. You can now login with your new password.</p>
            <a href="/login" class="login-link">Go to Login</a>
        </div>
    </div>

    <script>
        const form = document.getElementById('resetForm');
        const passwordInput = document.getElementById('password');
        const confirmInput = document.getElementById('confirmPassword');
        const passwordError = document.getElementById('passwordError');
        const confirmError = document.getElementById('confirmError');
        const apiError = document.getElementById('apiError');
        const submitBtn = document.getElementById('submitBtn');
        const successView = document.getElementById('successView');
        const token = '{token}';

        form.addEventListener('submit', async (e) => {{
            e.preventDefault();
            
            // Clear previous errors
            passwordError.classList.remove('show');
            confirmError.classList.remove('show');
            apiError.classList.remove('show');
            apiError.textContent = '';

            // Validate
            const password = passwordInput.value;
            const confirmPassword = confirmInput.value;
            let hasError = false;

            if (password.length < 8) {{
                passwordError.classList.add('show');
                hasError = true;
            }}

            if (password !== confirmPassword) {{
                confirmError.classList.add('show');
                hasError = true;
            }}

            if (hasError) return;

            // Submit
            submitBtn.disabled = true;
            submitBtn.textContent = 'Resetting...';

            try {{
                const response = await fetch('/api/auth/reset-password', {{
                    method: 'POST',
                    headers: {{
                        'Content-Type': 'application/json'
                    }},
                    body: JSON.stringify({{
                        token: token,
                        new_password: password
                    }})
                }});

                if (response.ok) {{
                    form.style.display = 'none';
                    successView.classList.add('show');
                }} else {{
                    const errorData = await response.json().catch(() => ({{}}));
                    apiError.textContent = errorData.error || 'Failed to reset password. The link may have expired.';
                    apiError.classList.add('show');
                    submitBtn.disabled = false;
                    submitBtn.textContent = 'Reset Password';
                }}
            }} catch (error) {{
                apiError.textContent = 'Network error. Please try again.';
                apiError.classList.add('show');
                submitBtn.disabled = false;
                submitBtn.textContent = 'Reset Password';
            }}
        }});

        // Real-time validation
        confirmInput.addEventListener('input', () => {{
            if (confirmInput.value && confirmInput.value !== passwordInput.value) {{
                confirmError.classList.add('show');
            }} else {{
                confirmError.classList.remove('show');
            }}
        }});
    </script>
</body>
</html>"#,
        token = token
    );

    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
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
