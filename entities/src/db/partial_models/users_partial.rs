use sea_orm::{prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

/// Partial user model for authentication purposes
/// Contains only the fields needed for login/authentication
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct UserAuthModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
}

/// Partial user model for listing/display purposes
/// Contains only public-facing user information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct UserPublicModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
}

/// Partial user model for basic identification
/// Contains only the minimum fields for user identification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct UserBasicModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

/// Partial user model for email verification checks
/// Contains only the fields needed to verify email status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct UserEmailVerificationModel {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
}
