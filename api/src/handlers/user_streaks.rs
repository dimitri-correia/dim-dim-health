use crate::{auth::middleware::RequireAuth, axummain::state::AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, response::Response};
use entities::sea_orm_active_enums::SubAppTypeEnum;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct StreakResponse {
    pub sub_app: String,
    pub current_streak: i32,
    pub longest_streak: i32,
}

/// Get all streaks for the authenticated user
pub async fn get_user_streaks(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Response {
    info!("Fetching streaks for user: {}", user.id);

    let streaks = match state
        .repositories
        .user_streak_repository
        .get_user_streaks(&user.id)
        .await
    {
        Ok(streaks) => streaks,
        Err(e) => {
            error!("Error fetching streaks: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch streaks"})),
            )
                .into_response();
        }
    };

    let response: Vec<StreakResponse> = streaks
        .into_iter()
        .map(|s| StreakResponse {
            sub_app: match s.sub_app {
                SubAppTypeEnum::Weight => "weight".to_string(),
                SubAppTypeEnum::Diet => "diet".to_string(),
                SubAppTypeEnum::Workout => "workout".to_string(),
            },
            current_streak: s.current_streak,
            longest_streak: s.longest_streak,
        })
        .collect();

    Json(response).into_response()
}

/// Update weight streak for the authenticated user
pub async fn update_weight_streak(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Response {
    info!("Updating weight streak for user: {}", user.id);

    let streak = match state
        .repositories
        .user_streak_repository
        .update_weight_streak(&user.id)
        .await
    {
        Ok(streak) => streak,
        Err(e) => {
            error!("Error updating weight streak: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update weight streak"})),
            )
                .into_response();
        }
    };

    Json(StreakResponse {
        sub_app: "weight".to_string(),
        current_streak: streak.current_streak,
        longest_streak: streak.longest_streak,
    })
    .into_response()
}

/// Update diet streak for the authenticated user
pub async fn update_diet_streak(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Response {
    info!("Updating diet streak for user: {}", user.id);

    let streak = match state
        .repositories
        .user_streak_repository
        .update_diet_streak(&user.id)
        .await
    {
        Ok(streak) => streak,
        Err(e) => {
            error!("Error updating diet streak: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update diet streak"})),
            )
                .into_response();
        }
    };

    Json(StreakResponse {
        sub_app: "diet".to_string(),
        current_streak: streak.current_streak,
        longest_streak: streak.longest_streak,
    })
    .into_response()
}
