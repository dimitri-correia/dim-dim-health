use crate::{
    auth::middleware::RequireAuth,
    axummain::state::AppState,
    schemas::user_group_schemas::JoinPublicGroupResponse,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entities::sea_orm_active_enums::UserGroup;
use log::error;
use tracing::info;

pub async fn join_public_group(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Json<JoinPublicGroupResponse>, impl IntoResponse> {
    info!("User {} requesting to join public group", user.id);

    // Check if user is already in the public group
    let user_groups = state
        .repositories
        .user_group_repository
        .find_by_user_id(&user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    // Check if user already has public group membership
    if user_groups
        .iter()
        .any(|ug| ug.group == UserGroup::PublicGroup)
    {
        info!(
            "User {} already a member of the public group",
            user.id
        );
        return Ok(Json(JoinPublicGroupResponse {
            message: "You are already a member of the public group".to_string(),
        }));
    }

    // Add user to public group
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
    Ok(Json(JoinPublicGroupResponse {
        message: "Successfully joined the public group".to_string(),
    }))
}
