use std::path::PathBuf;

use anyhow::anyhow;
use tauri::{command, State};
use whisper_rs::WhisperContextParameters;

use crate::{handlers::error::AppResult, services::AudioManager};

#[command]
pub async fn start_recording<'a>(manager: State<'a, AudioManager>) -> AppResult<()> {
    

    Ok(())
}

#[command]
pub async fn stop_recording<'a>(manager: State<'a, AudioManager>) -> AppResult<()> {
    Ok(())
}