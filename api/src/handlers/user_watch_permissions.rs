use crate::{
    auth::middleware::RequireAuth, axummain::state::AppState,
    schemas::user_watch_permission_schemas::*,
};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use entities::users;
use log::error;
use serde_json::json;
use tracing::info;
use validator::Validate;

/// Search for users by username (AJAX search with at least 3 characters)
pub async fn search_users(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<SearchUsersRequest>,
) -> Result<Json<SearchUsersResponse>, impl IntoResponse> {
    info!(
        "User {} searching for users with query: {}",
        user.id, params.query
    );

    if let Err(err) = params.validate() {
        info!("Validation error during user search: {}", err);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    let users = state
        .repositories
        .user_repository
        .search_by_username(&params.query)
        .await
        .map_err(|err| {
            error!("Failed to search users: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    let user_results: Vec<UserSearchResult> = users
        .into_iter()
        .filter(|u| u.id != user.id) // Exclude the current user from results
        .map(UserSearchResult::from)
        .collect();

    Ok(Json(SearchUsersResponse {
        users: user_results,
    }))
}

/// Get list of users that are watching me (people I allow to watch me)
pub async fn get_watchers(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Json<WatchersResponse>, StatusCode> {
    info!("User {} fetching list of watchers", user.id);

    let users: Vec<users::Model> = state
        .repositories
        .user_watch_permission_repository
        .find_all_watched(&user.id)
        .await
        .map_err(|err| {
            error!("Failed to fetch watchers: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let watchers: Vec<WatchPermissionWithUser> = users
        .into_iter()
        .map(|u| WatchPermissionWithUser {
            user_id: u.id,
            username: u.username.clone(),
        })
        .collect();

    Ok(Json(WatchersResponse { watchers }))
}

/// Get list of users I'm watching (people that allow me to watch them)
pub async fn get_watching(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Json<WatchingResponse>, StatusCode> {
    info!("User {} fetching list of users they are watching", user.id);

    let users: Vec<users::Model> = state
        .repositories
        .user_watch_permission_repository
        .find_all_watching(&user.id)
        .await
        .map_err(|err| {
            error!("Failed to fetch watching users: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let watching: Vec<WatchPermissionWithUser> = users
        .into_iter()
        .map(|u| WatchPermissionWithUser {
            user_id: u.id,
            username: u.username.clone(),
        })
        .collect();

    Ok(Json(WatchingResponse { watching }))
}

/// Grant watch permission to another user (allow them to watch me)
pub async fn grant_watch_permission(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<GrantWatchPermissionRequest>,
) -> Result<StatusCode, impl IntoResponse> {
    info!(
        "User {} granting watch permission to user {}",
        user.id, payload.user_id
    );

    if state
        .repositories
        .user_repository
        .find_by_id(&payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to check if user exists: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .is_none()
    {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found"})),
        )
            .into_response());
    }

    if state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to check existing permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .is_some()
    {
        return Ok(StatusCode::CONFLICT);
    }

    state
        .repositories
        .user_watch_permission_repository
        .create(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to create watch permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    Ok(StatusCode::CREATED)
}

pub async fn revoke_watch_permission(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<RevokeWatchPermissionRequest>,
) -> Result<StatusCode, impl IntoResponse> {
    info!(
        "User {} revoking watch permission from user {}",
        user.id, payload.user_id
    );

    if state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to check permission exists: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
        .is_none()
    {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Watch permission not found"})),
        )
            .into_response());
    }

    state
        .repositories
        .user_watch_permission_repository
        .delete_by_user_ids(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to delete watch permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    Ok(StatusCode::OK)
}
