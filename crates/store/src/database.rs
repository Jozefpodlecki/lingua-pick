use std::path::PathBuf;
use anyhow::Result;
use chrono::{DateTime, Utc};
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::{ManageConnection, Pool};
use uuid::Uuid;

use crate::{models::*, queries::*};

const MIGRATIONS: &[(&str, &str)] = &[
    (include_str!("migrations/001_init.sql"), "migrations/001_init.sql"),
    (include_str!("migrations/002_data.sql"), "migrations/002_data.sql")
];

pub struct DatabaseManager(Pool<DuckdbConnectionManager>);

impl DatabaseManager {
    pub fn new(path: PathBuf) -> Result<Self> {
        let manager = DuckdbConnectionManager::file(path)?;
        
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(manager)?;

        Ok(Self(pool))
    }

    pub fn ensure_created(&self) -> Result<()> {

        let connection = self.0.get()?;

        let table_exists: Option<bool> = connection
            .query_row(CHECK_MIGRATIONS_TABLE, [], |row| row.get(0))
            .ok()
            .filter(|pr| *pr);

        for (sql, file) in MIGRATIONS.iter() {
            let applied: Option<Uuid> = if table_exists.is_some() {
                connection
                    .query_row(GET_MIGRATION_BY_FILE_NAME, [file], |row| row.get(0))
                    .optional()?
            } else {
                None
            };

            if applied.is_some() {
                continue;
            }

            connection.execute_batch(sql)?;

            let params = params![Uuid::now_v7(), *file, Utc::now()];
            connection.execute(
                INSERT_MIGRATION,
                params,
            )?;
        }
        
        let version: &str = env!("CARGO_PKG_VERSION");
        connection.execute(UPDATE_METADATA, params![version, Utc::now()])?;

        Ok(())
    }

    pub fn pool(&self) -> &Pool<DuckdbConnectionManager> {
        &self.0
    }
}

pub struct UserRepository(Pool<DuckdbConnectionManager>);
pub struct UserProfileRepository(Pool<DuckdbConnectionManager>);
pub struct LanguageRepository(Pool<DuckdbConnectionManager>);

impl UserProfileRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: UserProfile) -> Result<()> {

        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.user_id,
            entity.created_on.naive_utc(),
            entity.updated_on.naive_utc(),
            entity.is_active,
            entity.source_language_id,
            entity.target_language_id
        ];

        connection.execute(INSERT_USER_PROFILE, params)?;

        Ok(())
    }

    pub fn set_active(&self, id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        connection.execute(UPDATE_USER_PROFILE_SET_ACTIVE, [id])?;

        Ok(())
    }

    pub fn get_by(&self, user_id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        connection.execute(GET_ACTIVE_USER_PROFILE_BY_USER, [user_id])?;

        Ok(())
    }
}

pub struct SessionRepository(Pool<DuckdbConnectionManager>);

impl UserRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: User) -> Result<User> {

        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.created_on.naive_utc(),
            entity.user_name,
            entity.password_hash
        ];

        connection.execute(INSERT_USER, params)?;

        Ok(entity)
    }

    pub fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        
        let connection = self.0.get()?;

        let result = connection
            .query_row(GET_USER_BY_USERNAME, [username], User::from_row)
            .optional()?;

        Ok(result)
    }
}

impl LanguageRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_all(&self) -> Result<Box<[Language]>> {
        let connection = self.0.get()?;

        let count: usize = connection.query_row(GET_LANGUAGES_COUNT, [], |row| row.get(0))?;

        let mut statement = connection.prepare(GET_LANGUAGES)?;
        let rows = statement.query_map([], Language::from_row)?;

        let mut languages = Vec::with_capacity(count);
        for lang in rows {
            languages.push(lang?);
        }

        Ok(languages.into_boxed_slice())
    }
}

impl SessionRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, session: Session) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            session.id,
            session.created_on.naive_utc()
        ];

        connection.execute(INSERT_SESSION, params)?;

        Ok(())
    }

    pub fn get(&self, id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        connection.execute(GET_SESSION_BY_ID, [id])?;

        Ok(())
    }
}