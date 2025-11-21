use crate::{
    auth::middleware::RequireAuth,
    axummain::state::AppState,
    schemas::user_watch_permission_schemas::*,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
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
    info!("User {} searching for users with query: {}", user.id, params.query);
    
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

    Ok(Json(SearchUsersResponse { users: user_results }))
}

/// Get list of users that are watching me (people I allow to watch me)
pub async fn get_watchers(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Json<WatchersResponse>, StatusCode> {
    info!("User {} fetching list of watchers", user.id);

    let permissions = state
        .repositories
        .user_watch_permission_repository
        .find_all_watched(&user.id)
        .await
        .map_err(|err| {
            error!("Failed to fetch watchers: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Batch load all watcher users
    let watcher_ids: Vec<_> = permissions
        .iter()
        .map(|p| p.user_watching_id)
        .collect();

    let users = state
        .repositories
        .user_repository
        .find_by_ids(&watcher_ids)
        .await
        .map_err(|err| {
            error!("Failed to fetch watcher users: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create a map for quick lookup
    let user_map: std::collections::HashMap<_, _> = users
        .into_iter()
        .map(|u| (u.id, u))
        .collect();

    // Build the response
    let watchers: Vec<_> = permissions
        .into_iter()
        .filter_map(|permission| {
            user_map.get(&permission.user_watching_id).map(|watcher_user| {
                WatchPermissionWithUser {
                    user_id: watcher_user.id,
                    username: watcher_user.username.clone(),
                    created_at: permission.created_at,
                }
            })
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

    let permissions = state
        .repositories
        .user_watch_permission_repository
        .find_all_watching(&user.id)
        .await
        .map_err(|err| {
            error!("Failed to fetch watching list: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Batch load all watched users
    let watched_ids: Vec<_> = permissions
        .iter()
        .map(|p| p.user_watched_id)
        .collect();

    let users = state
        .repositories
        .user_repository
        .find_by_ids(&watched_ids)
        .await
        .map_err(|err| {
            error!("Failed to fetch watched users: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create a map for quick lookup
    let user_map: std::collections::HashMap<_, _> = users
        .into_iter()
        .map(|u| (u.id, u))
        .collect();

    // Build the response
    let watching: Vec<_> = permissions
        .into_iter()
        .filter_map(|permission| {
            user_map.get(&permission.user_watched_id).map(|watched_user| {
                WatchPermissionWithUser {
                    user_id: watched_user.id,
                    username: watched_user.username.clone(),
                    created_at: permission.created_at,
                }
            })
        })
        .collect();

    Ok(Json(WatchingResponse { watching }))
}

/// Grant watch permission to another user (allow them to watch me)
pub async fn grant_watch_permission(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<GrantWatchPermissionRequest>,
) -> Result<Json<GrantWatchPermissionResponse>, impl IntoResponse> {
    info!("User {} granting watch permission to user {}", user.id, payload.user_id);

    // Check if the user to grant permission to exists
    let target_user = state
        .repositories
        .user_repository
        .find_by_id(&payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to check if user exists: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if target_user.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found"})),
        )
            .into_response());
    }

    // Check if permission already exists
    let existing_permission = state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to check existing permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if existing_permission.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "Watch permission already exists"})),
        )
            .into_response());
    }

    // Create the permission (user.id is watched, payload.user_id is watching)
    state
        .repositories
        .user_watch_permission_repository
        .create(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to create watch permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    Ok(Json(GrantWatchPermissionResponse {
        message: "Watch permission granted successfully".to_string(),
        user_watched_id: user.id,
        user_watching_id: payload.user_id,
    }))
}

/// Revoke watch permission from another user (stop allowing them to watch me)
pub async fn revoke_watch_permission(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<RevokeWatchPermissionRequest>,
) -> Result<Json<RevokeWatchPermissionResponse>, impl IntoResponse> {
    info!("User {} revoking watch permission from user {}", user.id, payload.user_id);

    // Check if permission exists
    let existing_permission = state
        .repositories
        .user_watch_permission_repository
        .find_by_user_ids(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to check permission exists: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if existing_permission.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Watch permission not found"})),
        )
            .into_response());
    }

    // Delete the permission
    state
        .repositories
        .user_watch_permission_repository
        .delete_by_user_ids(&user.id, &payload.user_id)
        .await
        .map_err(|err| {
            error!("Failed to delete watch permission: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    Ok(Json(RevokeWatchPermissionResponse {
        message: "Watch permission revoked successfully".to_string(),
    }))
}
