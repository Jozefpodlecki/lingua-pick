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
            user_name: row.get(2)?,
            password_hash: row.get(3)?,
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
    pub source_language_id: u32,
    pub target_language_id: u32
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
    pub id: u32,
    pub created_on: DateTime<Utc>,
    pub name: Box<str>,
    pub iso639_1: Option<Box<str>>,
    pub iso639_3: Box<str>,
}

impl Language {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            name: row.get(2)?,
            iso639_1: row.get(3)?,
            iso639_3: row.get(4)?,
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
    pub current_exercise_id: Option<Uuid>
}

impl Session {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            user_profile_id: row.get(1)?,
            created_on: row.get(2)?,
            updated_on: row.get(3)?,
            completed_on: row.get(4)?,
            exercise_count: row.get(5)?,
            total_exercise_count: row.get(6)?,
            current_exercise_id: row.get(7)?,
        })
    }
}

#[derive(Debug)]
pub struct Exercise {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
}

impl Exercise {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            
        })
    }
}

#[derive(Debug)]
pub struct ExerciseType {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub name: Box<str>,
    pub description: Box<str>,
    pub modality: Box<str>,
}

impl ExerciseType {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            modality: row.get(4)?,
        })
    }
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