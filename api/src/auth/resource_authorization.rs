use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

/// Extractor that verifies the authenticated user has permission to view a specific user's data
/// 
/// Usage in handlers:
/// ```
/// async fn get_user_data(
///     ViewUserData(target_user_id): ViewUserData,
///     RequireAuth(requesting_user): RequireAuth,
/// ) -> Result<Json<Response>, StatusCode> {
///     // target_user_id is guaranteed to be viewable by requesting_user
/// }
/// ```
#[derive(Debug)]
pub struct ViewUserData(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for ViewUserData
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // First, extract the authenticated user
        let RequireAuth(requesting_user) =
            RequireAuth::from_request_parts(parts, state)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Extract target user ID from path parameter
        let target_user_id = parts
            .extensions
            .get::<Uuid>()
            .copied()
            .ok_or(StatusCode::BAD_REQUEST)?;

        // Get app state
        let app_state = AppState::from_ref(state);

        // Check authorization
        let can_view = app_state
            .services
            .authorization
            .can_view_user_data(&requesting_user.id, &target_user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if !can_view {
            return Err(StatusCode::FORBIDDEN);
        }

        Ok(ViewUserData(target_user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::generate_token;
    use crate::repositories::user_watch_permission_repository::UserWatchPermissionRepository;
    use crate::services::Services;
    use axum::http::{HeaderMap, Method, Request, Version, request::Parts};
    use chrono::{FixedOffset, Utc};
    use entities::users::Model as User;
    use entities::user_watch_permissions;
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};

    fn create_mock_user(id: Uuid, email_verified: bool) -> User {
        let fixed_offset = FixedOffset::east_opt(0).expect("Invalid timezone offset");
        User {
            id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "password_hash".to_string(),
            created_at: Utc::now().with_timezone(&fixed_offset),
            updated_at: Utc::now().with_timezone(&fixed_offset),
            email_verified,
        }
    }

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

    async fn create_app_state(db: DatabaseConnection, jwt_secret: String) -> AppState {
        let redis = redis::Client::open("redis://localhost:6379")
            .unwrap()
            .get_connection_manager()
            .await
            .unwrap();
        AppState::new(db.clone(), redis, jwt_secret).await.unwrap()
    }

    fn create_request_parts_with_target_user(
        token: &str,
        target_user_id: Uuid,
    ) -> Parts {
        let mut req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .version(Version::HTTP_11)
            .body(())
            .unwrap();

        req.headers_mut().insert(
            "Authorization",
            format!("Token {}", token).parse().unwrap(),
        );

        let (mut parts, _body) = req.into_parts();
        parts.extensions.insert(target_user_id);
        parts
    }

    #[tokio::test]
    async fn test_view_user_data_own_data() {
        let user_id = Uuid::new_v4();
        let user = create_mock_user(user_id, true);

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .into_connection();

        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret.clone()).await;

        let token = generate_token(&user_id, &jwt_secret).unwrap();
        let mut parts = create_request_parts_with_target_user(&token, user_id);

        let result = ViewUserData::from_request_parts(&mut parts, &app_state).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, user_id);
    }

    #[tokio::test]
    async fn test_view_user_data_with_permission() {
        let watching_user_id = Uuid::new_v4();
        let watched_user_id = Uuid::new_v4();
        
        let watching_user = create_mock_user(watching_user_id, true);
        let permission = create_mock_permission(watched_user_id, watching_user_id);

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                vec![watching_user.clone()],
                vec![permission],
            ])
            .into_connection();

        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret.clone()).await;

        let token = generate_token(&watching_user_id, &jwt_secret).unwrap();
        let mut parts = create_request_parts_with_target_user(&token, watched_user_id);

        let result = ViewUserData::from_request_parts(&mut parts, &app_state).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, watched_user_id);
    }

    #[tokio::test]
    async fn test_view_user_data_without_permission() {
        let watching_user_id = Uuid::new_v4();
        let watched_user_id = Uuid::new_v4();
        
        let watching_user = create_mock_user(watching_user_id, true);

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                vec![watching_user.clone()],
                Vec::<user_watch_permissions::Model>::new(),
            ])
            .into_connection();

        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret.clone()).await;

        let token = generate_token(&watching_user_id, &jwt_secret).unwrap();
        let mut parts = create_request_parts_with_target_user(&token, watched_user_id);

        let result = ViewUserData::from_request_parts(&mut parts, &app_state).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::FORBIDDEN);
    }
}
