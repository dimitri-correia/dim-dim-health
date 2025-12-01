use crate::{
    auth::middleware::RequireVerifiedAuth, axummain::state::AppState,
    schemas::gym_schemas::*,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use entities::sea_orm_active_enums::MuscleEnum;
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// Query parameters
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct MuscleQuery {
    pub muscle: Option<MuscleEnum>,
    pub name: Option<String>,
}

// ============================================================================
// Gym Exercise handlers
// ============================================================================

pub async fn create_gym_exercise(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Json(payload): Json<CreateGymExerciseRequest>,
) -> Result<Json<GymExerciseResponse>, impl IntoResponse> {
    info!("Creating gym exercise for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    match state
        .repositories
        .gym_exercise_repository
        .create(
            payload.name,
            payload.description,
            payload.primary_muscles,
            payload.secondary_muscles,
            user.id,
        )
        .await
    {
        Ok(exercise) => Ok(Json(exercise)),
        Err(err) => {
            error!("Failed to create gym exercise: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_gym_exercises(
    State(state): State<AppState>,
    RequireVerifiedAuth(_user): RequireVerifiedAuth,
    Query(query): Query<MuscleQuery>,
) -> Result<Json<Vec<GymExerciseResponse>>, impl IntoResponse> {
    info!("Fetching gym exercises");

    let exercises = if let Some(muscle) = query.muscle {
        match state
            .repositories
            .gym_exercise_repository
            .find_by_muscle(muscle)
            .await
        {
            Ok(exercises) => exercises,
            Err(err) => {
                error!("Failed to fetch exercises by muscle: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    } else if let Some(name) = query.name {
        match state
            .repositories
            .gym_exercise_repository
            .find_by_name(&name)
            .await
        {
            Ok(exercises) => exercises,
            Err(err) => {
                error!("Failed to fetch exercises by name: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    } else {
        match state.repositories.gym_exercise_repository.find_all().await {
            Ok(exercises) => exercises,
            Err(err) => {
                error!("Failed to fetch exercises: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    };

    Ok(Json(exercises))
}

pub async fn get_gym_exercise(
    State(state): State<AppState>,
    RequireVerifiedAuth(_user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<Json<GymExerciseResponse>, impl IntoResponse> {
    info!("Fetching gym exercise: {}", id);

    match state
        .repositories
        .gym_exercise_repository
        .find_by_id_with_muscles(&id)
        .await
    {
        Ok(Some(exercise)) => Ok(Json(exercise)),
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym exercise: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn update_gym_exercise(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateGymExerciseRequest>,
) -> Result<Json<GymExerciseResponse>, impl IntoResponse> {
    info!("Updating gym exercise {} for user: {}", id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the exercise exists and was added by the user
    match state
        .repositories
        .gym_exercise_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(exercise)) => {
            if exercise.added_by != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym exercise: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_exercise_repository
        .update(
            id,
            payload.name,
            payload.description,
            payload.primary_muscles,
            payload.secondary_muscles,
        )
        .await
    {
        Ok(exercise) => Ok(Json(exercise)),
        Err(err) => {
            error!("Failed to update gym exercise: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_gym_exercise(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting gym exercise {} for user: {}", id, user.id);

    // Check if the exercise exists and was added by the user
    match state
        .repositories
        .gym_exercise_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(exercise)) => {
            if exercise.added_by != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym exercise: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_exercise_repository
        .delete(&id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete gym exercise: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

// ============================================================================
// Gym Session handlers
// ============================================================================

pub async fn create_gym_session(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Json(payload): Json<CreateGymSessionRequest>,
) -> Result<Json<GymSessionResponse>, impl IntoResponse> {
    info!("Creating gym session for user: {}", user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    match state
        .repositories
        .gym_session_repository
        .create(user.id, payload.date)
        .await
    {
        Ok(session) => Ok(Json(GymSessionResponse::from(session))),
        Err(err) => {
            error!("Failed to create gym session: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_gym_sessions(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<GymSessionResponse>>, impl IntoResponse> {
    info!("Fetching gym sessions for user: {}", user.id);

    let sessions = if let Some(date) = query.date {
        match state
            .repositories
            .gym_session_repository
            .find_by_user_and_date(&user.id, date)
            .await
        {
            Ok(sessions) => sessions,
            Err(err) => {
                error!("Failed to fetch sessions by date: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    } else {
        match state
            .repositories
            .gym_session_repository
            .find_by_user_id(&user.id)
            .await
        {
            Ok(sessions) => sessions,
            Err(err) => {
                error!("Failed to fetch sessions: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
            }
        }
    };

    let response: Vec<GymSessionResponse> = sessions.into_iter().map(GymSessionResponse::from).collect();
    Ok(Json(response))
}

pub async fn get_gym_session(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<Json<GymSessionResponse>, impl IntoResponse> {
    info!("Fetching gym session {} for user: {}", id, user.id);

    match state
        .repositories
        .gym_session_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
            Ok(Json(GymSessionResponse::from(session)))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn update_gym_session(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateGymSessionRequest>,
) -> Result<Json<GymSessionResponse>, impl IntoResponse> {
    info!("Updating gym session {} for user: {}", id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the session exists and belongs to the user
    match state
        .repositories
        .gym_session_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_session_repository
        .update(id, payload.date)
        .await
    {
        Ok(session) => Ok(Json(GymSessionResponse::from(session))),
        Err(err) => {
            error!("Failed to update gym session: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_gym_session(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, impl IntoResponse> {
    info!("Deleting gym session {} for user: {}", id, user.id);

    // Check if the session exists and belongs to the user
    match state
        .repositories
        .gym_session_repository
        .find_by_id(&id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_session_repository
        .delete(&id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete gym session: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

// ============================================================================
// Gym Set handlers
// ============================================================================

pub async fn create_gym_set(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<CreateGymSetRequest>,
) -> Result<Json<GymSetResponse>, impl IntoResponse> {
    info!("Creating gym set for session {} for user: {}", session_id, user.id);

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the session exists and belongs to the user
    match state
        .repositories
        .gym_session_repository
        .find_by_id(&session_id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    // Check if the exercise exists
    match state
        .repositories
        .gym_exercise_repository
        .find_by_id(&payload.exercise_id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Exercise not found"})),
            )
                .into_response());
        }
        Err(err) => {
            error!("Failed to fetch gym exercise: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_set_repository
        .create(
            session_id,
            payload.exercise_id,
            payload.set_number,
            payload.repetitions,
            payload.weight_kg,
        )
        .await
    {
        Ok(gym_set) => Ok(Json(GymSetResponse::from(gym_set))),
        Err(err) => {
            error!("Failed to create gym set: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn get_gym_sets(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path(session_id): Path<Uuid>,
) -> Result<Json<Vec<GymSetResponse>>, impl IntoResponse> {
    info!("Fetching gym sets for session {} for user: {}", session_id, user.id);

    // Check if the session exists and belongs to the user
    match state
        .repositories
        .gym_session_repository
        .find_by_id(&session_id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_set_repository
        .find_by_session_id(&session_id)
        .await
    {
        Ok(sets) => {
            let response: Vec<GymSetResponse> = sets.into_iter().map(GymSetResponse::from).collect();
            Ok(Json(response))
        }
        Err(err) => {
            error!("Failed to fetch gym sets: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn update_gym_set(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path((session_id, set_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateGymSetRequest>,
) -> Result<Json<GymSetResponse>, impl IntoResponse> {
    info!(
        "Updating gym set {} in session {} for user: {}",
        set_id, session_id, user.id
    );

    if let Err(err) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        )
            .into_response());
    }

    // Check if the session exists and belongs to the user
    match state
        .repositories
        .gym_session_repository
        .find_by_id(&session_id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    // Check if the set exists and belongs to this session
    match state
        .repositories
        .gym_set_repository
        .find_by_id(&set_id)
        .await
    {
        Ok(Some(set)) => {
            if set.session_id != session_id {
                return Err(StatusCode::NOT_FOUND.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym set: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state
        .repositories
        .gym_set_repository
        .update(set_id, payload.set_number, payload.repetitions, payload.weight_kg)
        .await
    {
        Ok(gym_set) => Ok(Json(GymSetResponse::from(gym_set))),
        Err(err) => {
            error!("Failed to update gym set: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn delete_gym_set(
    State(state): State<AppState>,
    RequireVerifiedAuth(user): RequireVerifiedAuth,
    Path((session_id, set_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, impl IntoResponse> {
    info!(
        "Deleting gym set {} from session {} for user: {}",
        set_id, session_id, user.id
    );

    // Check if the session exists and belongs to the user
    match state
        .repositories
        .gym_session_repository
        .find_by_id(&session_id)
        .await
    {
        Ok(Some(session)) => {
            if session.user_id != user.id {
                return Err(StatusCode::FORBIDDEN.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym session: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    // Check if the set exists and belongs to this session
    match state
        .repositories
        .gym_set_repository
        .find_by_id(&set_id)
        .await
    {
        Ok(Some(set)) => {
            if set.session_id != session_id {
                return Err(StatusCode::NOT_FOUND.into_response());
            }
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND.into_response()),
        Err(err) => {
            error!("Failed to fetch gym set: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    }

    match state.repositories.gym_set_repository.delete(&set_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!("Failed to delete gym set: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
