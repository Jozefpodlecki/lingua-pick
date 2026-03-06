use anyhow::anyhow;
use chrono::Utc;
use lingua_pick_store::{User, UserProfile, UserProfileRepository, UserRepository};
use rand::{distr::Alphanumeric, RngExt};
use tauri::{command, State};
use uuid::Uuid;

use crate::{context::AppContext, handlers::{error::{AppError, AppResult}, models::{AppUserProfile, LoadResult, LoginArgs}}, notifier::SetupEndedNotifier, services::{AppPasswordHasher, JwtClaims, JwtService}};

#[command]
pub async fn load_app<'a>(
    context: State<'a, AppContext>,
    setup_ended_notifier: State<'a, SetupEndedNotifier>) -> AppResult<LoadResult> {
    setup_ended_notifier.wait_loaded().await;

    let result = LoadResult {
        session_id: context.session_id
    };

    Ok(result)
}

pub fn generate_password(length: usize) -> String {
    let rng = rand::rng();

    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[command]
pub async fn login_with_windows<'a>(
    user_repository: State<'a, UserRepository>,
    password_hasher: State<'a, AppPasswordHasher>,
    jwt_service: State<'a, JwtService>,
) -> AppResult<String> {

    let user_name = whoami::username().map_err(|err| anyhow!(err))?.into_boxed_str();
    let user = user_repository.get_by_username(&user_name)?;

    let user = match user {
        Some(user) => user,
        None => {
            let password = generate_password(15);
            let password_hash = password_hasher.hash(&password)?;
            let user = User {
                id: Uuid::now_v7(),
                created_on: Utc::now(),
                user_name,
                password_hash
            };

            let user = user_repository.create(user)?;

            user
        },
    };

    let now = Utc::now().timestamp() as usize;
    let exp = now + 60 * 60 * 24;
    let claims = JwtClaims {
        sub: user.id,
        user_name: user.user_name,
        iss: "lingua-pick".into(),
        company: "lingua-pick".into(),
        iat: now,
        exp,
        role: None,
        session_id: None,
    };
    let token = jwt_service.encode(claims).map_err(|e| AppError::Command(anyhow::anyhow!("Could not generate JWT: {}", e)))?;

    Ok(token)
}

#[command]
pub async fn login_with_creds<'a>(
    args: LoginArgs,
    user_repository: State<'a, UserRepository>,
    password_hasher: State<'a, AppPasswordHasher>,
    jwt_service: State<'a, JwtService>,
) -> AppResult<String> {

    let LoginArgs {
        user_name,
        password
    } = args;

    let user = user_repository
        .get_by_username(&user_name)?
        .ok_or_else(|| AppError::Command(anyhow::anyhow!("Invalid user or password")))?;

    password_hasher
        .verify(password.as_bytes(), &user.password_hash)
        .map_err(|e| AppError::Command(anyhow::anyhow!("Invalid user or password: {}", e)))?;

    let now = Utc::now().timestamp() as usize;
    let exp = now + 60 * 60 * 24;
    let claims = JwtClaims {
        sub: user.id,
        user_name: user.user_name,
        iss: "lingua-pick".into(),
        company: "lingua-pick".into(),
        iat: now,
        exp,
        role: None,
        session_id: None,
    };
    let token = jwt_service.encode(claims).map_err(|e| AppError::Command(anyhow::anyhow!("Could not generate JWT: {}", e)))?;

    Ok(token)
}

