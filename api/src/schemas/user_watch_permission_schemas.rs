use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct SearchUsersRequest {
    #[validate(length(min = 3, message = "Search query must be at least 3 characters"))]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    pub users: Vec<UserSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchPermissionWithUser {
    pub user_id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchersResponse {
    pub watchers: Vec<WatchPermissionWithUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchingResponse {
    pub watching: Vec<WatchPermissionWithUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrantWatchPermissionRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeWatchPermissionRequest {
    pub user_id: Uuid,
}
