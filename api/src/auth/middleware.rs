use crate::{auth::jwt::validate_token, axummain::state::AppState};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{HeaderMap, StatusCode, request::Parts},
};
use entities::users::Model as User;
use uuid::Uuid;

// For protected routes - requires valid JWT
#[derive(Debug)]
pub struct RequireAuth(pub User);

// For optional auth - extracts user if token present
#[derive(Debug)]
pub struct OptionalAuth(pub Option<User>);

impl<S> FromRequestParts<S> for RequireAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let headers = &parts.headers;
        let token = extract_token_from_headers(headers).ok_or(StatusCode::UNAUTHORIZED)?;

        let claims =
            validate_token(&token, &app_state.jwt_secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user = app_state
            .repositories
            .user_repository
            .find_by_id(&user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(RequireAuth(user))
    }
}

impl<S> FromRequestParts<S> for OptionalAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let headers = &parts.headers;
        let token = match extract_token_from_headers(headers) {
            Some(token) => token,
            None => return Ok(OptionalAuth(None)),
        };

        let claims = match validate_token(&token, &app_state.jwt_secret) {
            Ok(claims) => claims,
            Err(_) => return Ok(OptionalAuth(None)),
        };

        let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => return Ok(OptionalAuth(None)),
        };

        let user = app_state
            .repositories
            .user_repository
            .find_by_id(&user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(OptionalAuth(user))
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;

    auth_header.strip_prefix("Token ").map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, Method, Request, Version, request::Parts};
    use chrono::{FixedOffset, Utc};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};

    fn create_mock_user() -> User {
        let fixed_offset = FixedOffset::east_opt(0).expect("Invalid timezone offset");
        User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "password_hash".to_string(),
            created_at: Utc::now().with_timezone(&fixed_offset),
            updated_at: Utc::now().with_timezone(&fixed_offset),
            email_verified: true,
        }
    }

    fn create_mock_db(user: &User) -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .into_connection()
    }

    async fn create_app_state(db: DatabaseConnection, jwt_secret: String) -> AppState {
        let redis = redis::Client::open("redis://localhost:6379")
            .unwrap()
            .get_connection_manager()
            .await
            .unwrap();
        AppState::new(db.clone(), redis, jwt_secret).await.unwrap()
    }

    fn create_request_parts(token: Option<&str>) -> Parts {
        // Start from a dummy Request
        let mut req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .version(Version::HTTP_11)
            .body(())
            .unwrap();

        // Insert Authorization header if token exists
        if let Some(token_str) = token {
            req.headers_mut().insert(
                "Authorization",
                format!("Token {}", token_str).parse().unwrap(),
            );
        }

        // Decompose into Parts
        let (parts, _body) = req.into_parts();
        parts
    }

    #[tokio::test]
    async fn test_require_auth_valid_token() {
        let user = create_mock_user();
        let db = create_mock_db(&user);
        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret.clone()).await;

        let token = crate::auth::jwt::generate_token(&user.id, &jwt_secret)
            .expect("Failed to generate token");

        let mut parts = create_request_parts(Some(&token));

        let auth = RequireAuth::from_request_parts(&mut parts, &app_state).await;
        assert!(auth.is_ok());
    }

    #[tokio::test]
    async fn test_require_auth_invalid_token() {
        let user = create_mock_user();
        let db = create_mock_db(&user);
        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret).await;

        let mut parts = create_request_parts(Some("invalid.token.here"));

        let auth = RequireAuth::from_request_parts(&mut parts, &app_state).await;
        assert_eq!(auth.unwrap_err(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_require_auth_missing_token() {
        let user = create_mock_user();
        let db = create_mock_db(&user);
        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret).await;

        let mut parts = create_request_parts(None);

        let auth = RequireAuth::from_request_parts(&mut parts, &app_state).await;
        assert_eq!(auth.unwrap_err(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_optional_auth_valid_token() {
        let user = create_mock_user();
        let db = create_mock_db(&user);
        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret.clone()).await;

        let token = crate::auth::jwt::generate_token(&user.id, &jwt_secret)
            .expect("Failed to generate token");

        let mut parts = create_request_parts(Some(&token));

        let auth = OptionalAuth::from_request_parts(&mut parts, &app_state).await;
        assert!(auth.is_ok());
        assert!(auth.unwrap().0.is_some());
    }

    #[tokio::test]
    async fn test_optional_auth_no_token() {
        let user = create_mock_user();
        let db = create_mock_db(&user);
        let jwt_secret = "test_secret".to_string();
        let app_state = create_app_state(db, jwt_secret).await;

        let mut parts = create_request_parts(None);

        let auth = OptionalAuth::from_request_parts(&mut parts, &app_state).await;
        assert!(auth.is_ok());
        assert!(auth.unwrap().0.is_none());
    }

    #[tokio::test]
    async fn test_extract_token_from_headers() {
        let headers = HeaderMap::new();
        assert_eq!(extract_token_from_headers(&headers), None);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Token test.token.here".parse().unwrap());
        assert_eq!(
            extract_token_from_headers(&headers),
            Some("test.token.here".to_string())
        );

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer test.token.here".parse().unwrap());
        assert_eq!(extract_token_from_headers(&headers), None);
    }
}
