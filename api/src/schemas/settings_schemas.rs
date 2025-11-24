use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSettingsRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    pub profile_image: Option<entities::sea_orm_active_enums::UserProfileImage>,

    #[validate(nested)]
    pub passwords: Option<PasswordChange>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PasswordChange {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateSettingsResponse {
    pub message: String,
}
