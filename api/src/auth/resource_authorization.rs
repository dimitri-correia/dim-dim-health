use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
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
    // Note: Integration tests for ViewUserData extractor are complex because they require
    // mocking multiple database queries (user lookup + permission check). The core authorization
    // logic is thoroughly tested in the user_view_authorization module tests.
    // 
    // For real-world testing of this extractor, use integration tests with a test database.
}
