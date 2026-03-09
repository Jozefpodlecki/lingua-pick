use anyhow::anyhow;
use chrono::Utc;
use lingua_pick_store::{Language, LanguageRepository, User, UserProfile, UserProfileRepository, UserRepository};
use rand::{distr::Alphanumeric, RngExt};
use tauri::{command, State};
use uuid::Uuid;

use crate::{context::AppContext, handlers::{error::{AppError, AppResult}, models::{AppUserProfile, LoadResult, LoginArgs}}, notifier::SetupEndedNotifier, services::{AppPasswordHasher, JwtClaims, JwtService}};

#[command]
pub async fn get_current_profile<'a>(
    token: String, 
    profile_repository: State<'a, UserProfileRepository>,
    jwt_service: State<'a, JwtService>,) -> AppResult<Option<AppUserProfile>> {
    
    let token = jwt_service.decode(&token).map_err(|err| AppError::Command(anyhow::anyhow!("Could not decode JWT: {}", err)))?;
    
    let user_id = token.claims.sub;
    let profile = profile_repository.get_active_by_user(user_id)?
        .map(AppUserProfile::from);
    
    Ok(profile)
}

#[command]
pub async fn get_all_profiles<'a>(
    token: String, 
    profile_repository: State<'a, UserProfileRepository>,
    jwt_service: State<'a, JwtService>,) -> AppResult<Vec<AppUserProfile>> {
    
    let token = jwt_service.decode(&token).map_err(|err| AppError::Command(anyhow::anyhow!("Could not decode JWT: {}", err)))?;
    
    let user_id = token.claims.sub;
    let profile = profile_repository.get_by_user(user_id)?
        .into_iter()
        .cloned()
        .map(AppUserProfile::from)
        .collect();
    
    Ok(profile)
}

#[command]
pub async fn get_all_languages<'a>(
    token: String, 
    lang_repository: State<'a, LanguageRepository>,
    jwt_service: State<'a, JwtService>,) -> AppResult<Box<[Language]>> {
    
    let token = jwt_service.decode(&token).map_err(|err| AppError::Command(anyhow::anyhow!("Could not decode JWT: {}", err)))?;
    
    let user_id = token.claims.sub;
    let items = lang_repository.get_all()?;
    
    Ok(items)
}

#[command]
pub async fn get_default_language<'a>(
    token: String, 
    lang_repository: State<'a, LanguageRepository>,
    jwt_service: State<'a, JwtService>,) -> AppResult<Option<Language>> {
    
    let token = jwt_service.decode(&token)
        .map_err(|err| AppError::Command(anyhow::anyhow!("Could not decode JWT: {}", err)))?;

    let lang = whoami::lang_prefs()
        .map_err(|err| AppError::Command(anyhow::anyhow!("Could retrieve language preferences: {}", err)))?;

    
    for lang in lang.message_langs() {
       let tag = lang.to_string();
        let iso639 = tag.split('-').next().unwrap_or(&tag);
        println!("{iso639}");
    }

    let items = lang_repository.get_by_iso639_3("en")?;
    
    Ok(items)
}