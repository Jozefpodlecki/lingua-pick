use chrono::{DateTime, Utc};
use duckdb::Row;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub user_name: Box<str>,
    pub password_hash: Vec<u8>
}

impl User {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            user_name: row.get::<usize, String>(2)?.into_boxed_str(),
            password_hash: row.get::<usize, Vec<u8>>(3)?,
        })
    }
}

#[derive(Debug)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub is_active: bool,
    pub source_language_id: Uuid,
    pub target_language_id: Uuid
}

#[derive(Debug)]
pub struct Language {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub name: Box<str>,
    pub iso2: Option<Box<str>>,
    pub iso3: Box<str>,
}

impl Language {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Language {
            id: row.get(0)?,
            created_on: row.get(1)?,
            name: row.get::<usize, String>(2)?.into_boxed_str(),
            iso2: row.get::<usize, Option<String>>(3)?.map(|s| s.into_boxed_str()),
            iso3: row.get::<usize, String>(4)?.into_boxed_str(),
        })
    }
}

#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub completed_on: Option<DateTime<Utc>>,
    pub source_language_id: DateTime<Utc>,
    pub target_language_id: DateTime<Utc>,
    pub exercise_count: u8
}