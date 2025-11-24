use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::guest_name_generator::GUEST_EMAIL_DOMAIN;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub user: RegisterUserData,
}

#[derive(Deserialize, Validate)]
pub struct RegisterUserData {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub profile_image: Option<entities::sea_orm_active_enums::UserProfileImage>,
}

impl std::fmt::Debug for RegisterUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterUserData")
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub user: LoginUserData,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

impl std::fmt::Debug for LoginUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginUserData")
            .field("email", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserData,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub username: String,
    pub email_verified: bool,
    pub created_at: DateTime<FixedOffset>,
    pub is_guest: bool,
    pub profile_image: entities::sea_orm_active_enums::UserProfileImage,
}

impl UserData {
    pub fn from_user(user: entities::users::Model) -> Self {
        // a bit of a hack, cleaner to use the user_groups table but this will do for now
        let is_guest = user.email.ends_with(GUEST_EMAIL_DOMAIN);

        Self {
            email: user.email,
            username: user.username,
            email_verified: user.email_verified,
            created_at: user.created_at,
            is_guest,
            profile_image: user.profile_image,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_user_data_debug_redacts_password() {
        let data = RegisterUserData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "supersecretpassword".to_string(),
            profile_image: None,
        };

        let debug_output = format!("{:?}", data);

        // Password should be redacted
        assert!(debug_output.contains("[REDACTED]"));
        // Password should NOT be visible
        assert!(!debug_output.contains("supersecretpassword"));
        // Other fields should be visible
        assert!(debug_output.contains("testuser"));
        assert!(debug_output.contains("test@example.com"));
    }

    #[test]
    fn test_login_user_data_debug_redacts_password() {
        let data = LoginUserData {
            email: "test@example.com".to_string(),
            password: "mysecretpassword".to_string(),
        };

        let debug_output = format!("{:?}", data);

        // Password should be redacted
        assert!(debug_output.contains("[REDACTED]"));
        // Password should NOT be visible
        assert!(!debug_output.contains("mysecretpassword"));
        // Email should be visible
        assert!(debug_output.contains("test@example.com"));
    }
}
