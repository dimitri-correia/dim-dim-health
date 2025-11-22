use crate::{
    auth::middleware::RequireVerifiedAuth,
    axummain::state::AppState,
    schemas::user_group_schemas::{UserGroupMembersResponse, UserGroupResponse},
};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use entities::sea_orm_active_enums::UserGroup;
use log::error;
use tracing::info;

pub async fn join_public_group(
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    State(state): State<AppState>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("User {} requesting to join public group", user.id);

    if state
        .repositories
        .user_group_repository
        .is_user_id_in_group(&user.id, UserGroup::PublicGroup)
        .await
        .map_err(|e| {
            error!(
                "Error checking membership for user {} in public group: {}",
                user.id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?
    {
        info!("User {} already a member of the public group", user.id);
        return Ok(StatusCode::OK);
    }

    if let Err(err) = state
        .repositories
        .user_group_repository
        .create(&user.id, UserGroup::PublicGroup)
        .await
    {
        error!("Failed to add user {} to public group: {}", user.id, err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    info!("Successfully added user {} to public group", user.id);
    Ok(StatusCode::OK)
}

pub async fn leave_public_group(
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    State(state): State<AppState>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("User {} requesting to leave public group", user.id);

    if let Err(err) = state
        .repositories
        .user_group_repository
        .delete_by_user_id_and_group(&user.id, UserGroup::PublicGroup)
        .await
    {
        error!(
            "Failed to remove user {} from public group: {}",
            user.id, err
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    info!("Successfully removed user {} from public group", user.id);
    Ok(StatusCode::OK)
}

pub async fn get_user_groups(
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    State(state): State<AppState>,
) -> Result<Json<UserGroupResponse>, impl IntoResponse> {
    info!("Fetching groups for user: {}", user.id);

    match state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
    {
        Ok(groups) => Ok(Json(UserGroupResponse {
            groups: groups
                .into_iter()
                .map(|g| format!("{:?}", g.group))
                .collect(),
        })),
        Err(err) => {
            error!("Failed to fetch user groups: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_public_group_members(
    State(state): State<AppState>,
) -> Result<Json<UserGroupMembersResponse>, impl IntoResponse> {
    info!("Fetching members of the public group");

    match state
        .repositories
        .user_group_repository
        .find_all_in_group(UserGroup::PublicGroup)
        .await
    {
        Ok(members) => Ok(Json(UserGroupMembersResponse {
            users: members.into_iter().map(|m| m.user_id).collect(),
        })),
        Err(err) => {
            error!("Failed to fetch public group members: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
