use crate::repositories::user_watch_permission_repository::UserWatchPermissionRepository;
use uuid::Uuid;

/// Service for checking if a user can view another user's data
#[derive(Clone)]
pub struct UserViewAuthorization {
    watch_permission_repo: UserWatchPermissionRepository,
}

impl UserViewAuthorization {
    pub fn new(watch_permission_repo: UserWatchPermissionRepository) -> Self {
        Self {
            watch_permission_repo,
        }
    }

    /// Check if the requesting user can view the target user's data
    /// Returns true if:
    /// - The requesting user is viewing their own data (requesting_user_id == target_user_id)
    /// - The requesting user has watch permission for the target user
    pub async fn can_view_user_data(
        &self,
        requesting_user_id: &Uuid,
        target_user_id: &Uuid,
    ) -> Result<bool, sea_orm::DbErr> {
        // Users can always view their own data
        if requesting_user_id == target_user_id {
            return Ok(true);
        }

        // Check if requesting user has permission to watch the target user
        let permission = self
            .watch_permission_repo
            .find_by_user_ids(target_user_id, requesting_user_id)
            .await?;

        Ok(permission.is_some())
    }

    /// Get all user IDs that the requesting user can view (including themselves)
    pub async fn get_viewable_user_ids(
        &self,
        requesting_user_id: &Uuid,
    ) -> Result<Vec<Uuid>, sea_orm::DbErr> {
        let mut viewable_ids = vec![*requesting_user_id];

        // Get all users that the requesting user is watching
        let watched_permissions = self
            .watch_permission_repo
            .find_all_watching(requesting_user_id)
            .await?;

        for permission in watched_permissions {
            viewable_ids.push(permission.user_watched_id);
        }

        Ok(viewable_ids)
    }

    /// Verify that the requesting user can view the target user's data
    /// Returns Ok(()) if authorized, Err otherwise
    pub async fn verify_view_permission(
        &self,
        requesting_user_id: &Uuid,
        target_user_id: &Uuid,
    ) -> Result<(), ViewAuthorizationError> {
        let can_view = self
            .can_view_user_data(requesting_user_id, target_user_id)
            .await
            .map_err(|e| ViewAuthorizationError::DatabaseError(e.to_string()))?;

        if can_view {
            Ok(())
        } else {
            Err(ViewAuthorizationError::Forbidden)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewAuthorizationError {
    Forbidden,
    DatabaseError(String),
}

impl std::fmt::Display for ViewAuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewAuthorizationError::Forbidden => {
                write!(f, "You do not have permission to view this user's data")
            }
            ViewAuthorizationError::DatabaseError(msg) => {
                write!(f, "Database error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ViewAuthorizationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user_watch_permission_repository::UserWatchPermissionRepository;
    use chrono::Utc;
    use entities::user_watch_permissions;
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult};

    fn create_mock_permission(
        user_watched_id: Uuid,
        user_watching_id: Uuid,
    ) -> user_watch_permissions::Model {
        user_watch_permissions::Model {
            user_watched_id,
            user_watching_id,
            created_at: Utc::now().into(),
        }
    }

    #[tokio::test]
    async fn test_can_view_own_data() {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let user_id = Uuid::new_v4();
        let result = auth.can_view_user_data(&user_id, &user_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_can_view_with_permission() {
        let user_watching = Uuid::new_v4();
        let user_watched = Uuid::new_v4();
        let permission = create_mock_permission(user_watched, user_watching);

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![permission]])
            .into_connection();

        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let result = auth
            .can_view_user_data(&user_watching, &user_watched)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_cannot_view_without_permission() {
        let user_watching = Uuid::new_v4();
        let user_watched = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![Vec::<user_watch_permissions::Model>::new()])
            .into_connection();

        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let result = auth
            .can_view_user_data(&user_watching, &user_watched)
            .await;

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_get_viewable_user_ids_includes_self() {
        let user_id = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![Vec::<user_watch_permissions::Model>::new()])
            .into_connection();

        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let result = auth.get_viewable_user_ids(&user_id).await;

        assert!(result.is_ok());
        let ids = result.unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0], user_id);
    }

    #[tokio::test]
    async fn test_get_viewable_user_ids_includes_watched_users() {
        let user_watching = Uuid::new_v4();
        let user_watched_1 = Uuid::new_v4();
        let user_watched_2 = Uuid::new_v4();

        let permission_1 = create_mock_permission(user_watched_1, user_watching);
        let permission_2 = create_mock_permission(user_watched_2, user_watching);

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![permission_1, permission_2]])
            .into_connection();

        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let result = auth.get_viewable_user_ids(&user_watching).await;

        assert!(result.is_ok());
        let ids = result.unwrap();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&user_watching));
        assert!(ids.contains(&user_watched_1));
        assert!(ids.contains(&user_watched_2));
    }

    #[tokio::test]
    async fn test_verify_view_permission_success() {
        let user_id = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let result = auth.verify_view_permission(&user_id, &user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_view_permission_forbidden() {
        let user_watching = Uuid::new_v4();
        let user_watched = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![Vec::<user_watch_permissions::Model>::new()])
            .into_connection();

        let repo = UserWatchPermissionRepository::new(db);
        let auth = UserViewAuthorization::new(repo);

        let result = auth
            .verify_view_permission(&user_watching, &user_watched)
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ViewAuthorizationError::Forbidden);
    }
}
