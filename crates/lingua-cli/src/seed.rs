use chrono::Utc;
use duckdb_neo::{environment::{Environment, StorageLocation}, r2d2::ConnectionManager, statement::Statement};
use uuid::Uuid;
use thiserror::Error;

use crate::{api::*, migration::{MigrationError, apply_migrations}, store::*, types::{Session, User, UserStats}, utils::*};

#[derive(Debug, Error)]
pub enum SeedError {
    #[error("database error: {0}")]
    Database(#[from] duckdb_neo::error::Error),

    #[error("database pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("migration error: {0}")]
    Migration(#[from] MigrationError),

    #[error("store error: {0}")]
    Store(#[from] StoreError),

    #[error("password hash error: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),

    #[error("seed language not found: {0}")]
    LanguageNotFound(String),
}

pub struct SeedResult {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub lang_store: LanguageStore,
    pub user_store: UserStore,
    pub user_stats_store: UserStatsStore,
    pub exercise_store: ExerciseStore,
    pub exercise_definition_store: ExerciseDefinitionStore,
    pub session_store: SessionStore,
}

pub fn create_db() -> Result<r2d2::Pool<ConnectionManager>, SeedError> {
    let env = Environment::new()?;
    let db = env.open(StorageLocation::InMemory)?;

    let pool = r2d2::Pool::builder()
        .max_size(2)
        .build(ConnectionManager::new(&db))?;

    Ok(pool)
}

pub fn generate(user_name: &str, lang_id: &str, target_lang_id: &str) -> Result<SeedResult, SeedError> {
    let pool = create_db()?;

    let connection = pool.get()?;
    apply_migrations(&connection)?;

    let lang_store = LanguageStore::new(pool.clone());
    let user_store = UserStore::new(pool.clone());
    let user_stats_store = UserStatsStore::new(pool.clone());
    let exercise_store = ExerciseStore::new(pool.clone());
    let session_store = SessionStore::new(pool.clone());
    let exercise_definition_store = ExerciseDefinitionStore::new(pool.clone());

    let now = Utc::now();

    lang_store.insert_json("assets/languages.json")?;

    let lang = lang_store
        .get_by_id()?
        .ok_or_else(|| SeedError::LanguageNotFound(target_lang_id.into()))?;

    let user = User {
        id: Uuid::now_v7(),
        source_language_id: lang_id.into(),
        username: user_name.into(),
        password_hash: hash_password("123")?,
        created_on: now,
        updated_on: now,
    };

    user_store.insert(&user)?;

    let stats = UserStats {
        user_id: user.id,
        target_language_id: lang.id.clone(),
        created_on: now,
        updated_on: now,
        struggling_categories: vec![],
    };

    user_stats_store.insert(&stats)?;

    let session = Session {
        id: Uuid::now_v7(),
        user_id: user.id,
        target_language_id: lang.id,
        created_on: now,
        exercise_count: 0,
        max_exercise_count: 10,
    };

    session_store.insert(&session)?;

    Ok(SeedResult {
        user_id: user.id,
        session_id: session.id,
        lang_store,
        user_store,
        user_stats_store,
        exercise_store,
        exercise_definition_store,
        session_store,
    })
}