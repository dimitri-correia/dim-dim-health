use sea_orm::{prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

/// Partial refresh token model for validation
/// Contains only the fields needed for token validation
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct RefreshTokenValidationModel {
    pub user_id: Uuid,
    pub expires_at: DateTimeWithTimeZone,
    pub used_at: Option<DateTimeWithTimeZone>,
}

/// Partial email verification token model for validation
/// Contains only the fields needed for token validation
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct EmailVerificationTokenValidationModel {
    pub user_id: Uuid,
    pub expires_at: DateTimeWithTimeZone,
}
