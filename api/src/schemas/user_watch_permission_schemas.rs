use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Response for a single user (used in search results)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub id: Uuid,
    pub username: String,
}

impl From<entities::users::Model> for UserSearchResult {
    fn from(user: entities::users::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
        }
    }
}

/// Request to search users by username
#[derive(Debug, Deserialize, Validate)]
pub struct SearchUsersRequest {
    #[validate(length(min = 3, message = "Search query must be at least 3 characters"))]
    pub query: String,
}

/// Response for user search
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    pub users: Vec<UserSearchResult>,
}

/// Response for a watch permission with user details
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchPermissionWithUser {
    pub user_id: Uuid,
    pub username: String,
    pub created_at: DateTime<FixedOffset>,
}

/// Response for getting list of people watching me (people I allow)
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchersResponse {
    pub watchers: Vec<WatchPermissionWithUser>,
}

/// Response for getting list of people I'm watching (people that allow me)
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchingResponse {
    pub watching: Vec<WatchPermissionWithUser>,
}

/// Request to grant watch permission to a user
#[derive(Debug, Deserialize)]
pub struct GrantWatchPermissionRequest {
    pub user_id: Uuid,
}

/// Request to revoke watch permission from a user
#[derive(Debug, Deserialize)]
pub struct RevokeWatchPermissionRequest {
    pub user_id: Uuid,
}

/// Response when granting watch permission
#[derive(Debug, Serialize, Deserialize)]
pub struct GrantWatchPermissionResponse {
    pub user_watched_id: Uuid,
    pub user_watching_id: Uuid,
    pub created_at: DateTime<FixedOffset>,
}

/// Response when revoking watch permission
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeWatchPermissionResponse {
    pub user_watched_id: Uuid,
    pub user_watching_id: Uuid,
    pub message: String,
}
