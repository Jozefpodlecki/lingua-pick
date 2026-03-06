use chrono::{DateTime, Utc};
use duckdb::Row;
use serde::Serialize;
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

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub is_active: bool,
    pub source_language_id: Uuid,
    pub target_language_id: Uuid
}

impl UserProfile {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Self {
            id: row.get(0)?,
            user_id: row.get(1)?,
            created_on: row.get(2)?,
            updated_on: row.get(3)?,
            is_active: row.get(4)?,
            source_language_id: row.get(5)?,
            target_language_id: row.get(6)?,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub name: Box<str>,
    pub iso639_1: Option<Box<str>>,
    pub iso639_3: Box<str>,
}

impl Language {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Language {
            id: row.get(0)?,
            created_on: row.get(1)?,
            name: row.get::<usize, String>(2)?.into_boxed_str(),
            iso639_1: row.get::<usize, Option<String>>(3)?.map(|s| s.into_boxed_str()),
            iso639_3: row.get::<usize, String>(4)?.into_boxed_str(),
        })
    }
}

#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_profile_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub completed_on: Option<DateTime<Utc>>,
    pub exercise_count: u8,
    pub total_exercise_count: u8,
}

#[derive(Debug)]
pub struct Exercise {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ExerciseType {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub name: Box<str>,
    pub description: Box<str>,
    pub modality: Box<str>,
}


#[derive(Debug)]
pub struct Lexeme {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct Character {
    pub id: Uuid,
}

impl Character {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Character {
            id: row.get(0)?,
            
        })
    }
}