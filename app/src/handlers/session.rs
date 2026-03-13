use anyhow::anyhow;
use chrono::Utc;
use lingua_pick_store::*;
use tauri::{command, State};
use uuid::Uuid;

use crate::{context::AppContext, handlers::error::{AppError, AppResult}};

#[command]
pub async fn get_session<'a>(
    token: String,
    id: Uuid,
    session_repo: State<'a, SessionRepository>) -> AppResult<Option<Session>> {
    
    let session = session_repo
        .get_by_id(id)
        .map_err(|err| AppError::Command(anyhow::anyhow!("Could not fetch session: {}", err)))?;

    Ok(session)
}

#[command]
pub async fn get_exercise<'a>(
    token: String,
    id: u32,
    exercise_repo: State<'a, ExerciseRepository>) -> AppResult<Option<Exercise>> {
    
    let exercise = exercise_repo
        .get_by_id(id)
        .map_err(|err| AppError::Command(anyhow::anyhow!("Could not fetch exercise: {}", err)))?;

    Ok(exercise)
}