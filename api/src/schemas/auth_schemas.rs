use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub user: RegisterUserData,
}

#[derive(Debug, Deserialize, Validate)]
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
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub user: LoginUserData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
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
}

impl UserData {
    pub fn from_user(user: entities::users::Model) -> Self {
        Self {
            email: user.email,
            username: user.username,
        }
    }
}
