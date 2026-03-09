use chrono::{Date, DateTime, NaiveDate, Utc};
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
    pub iso639_3: Option<Box<str>>,
    pub ietf_bcp_47: Box<str>,
    pub is_dialect: bool,
    pub is_regional_standard: bool,
    pub is_macro_language: bool,
    pub parent_id: Option<u32>
}

impl Language {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            name: row.get(2)?,
            iso639_1: row.get(3)?,
            iso639_3: row.get(4)?,
            ietf_bcp_47: row.get(5)?,
            is_dialect: row.get(6)?,
            is_regional_standard: row.get(7)?,
            is_macro_language: row.get(8)?,
            parent_id: row.get(9)?,
        })
    }
}

pub struct LanguageAsset {
    pub id: u32,
    pub created_on: DateTime<Utc>,
    pub file_name: Box<str>,
    pub language_id: u32,
}

impl LanguageAsset {
    pub fn kind(&self) -> &'static str {
        
        if self.file_name.contains("lexemes.json") {
            return "lexemes";
        }

        if self.file_name.contains("graphemes.json") {
            return "graphemes";
        }

        "unknown"
    }

    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            file_name: row.get(2)?,
            language_id: row.get(3)?,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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
    pub id: u32,
    pub type_id: u8,
    pub session_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub completed_on: Option<DateTime<Utc>>,
    pub data: Box<str>,
    pub result: Option<Box<str>>,
    pub verdict: Option<Box<str>>,
}

impl Exercise {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            type_id: row.get(1)?,
            session_id: row.get(2)?,
            created_on: row.get(3)?,
            completed_on: row.get(4)?,
            data: row.get(5)?,
            result: row.get(6)?,
            verdict: row.get(7)?
        })
    }
}

#[derive(Debug)]
pub struct ExerciseType {
    pub id: u8,
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
    pub id: u32,
    pub created_on: DateTime<Utc>,
    pub text: Box<str>,
    pub language_id: u32,
    pub normalized: Box<str>,
}

impl Lexeme {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {

        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            text: row.get(2)?,
            language_id: row.get(3)?,
            normalized: row.get(4)?,
        })
    }
}

#[derive(Debug)]
pub struct LexemeReading {
    pub id: u32,
    pub lexeme_id: u32,
    pub system: Box<str>,
    pub value: Box<str>,
}

impl LexemeReading {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            lexeme_id: row.get(1)?,
            system: row.get(2)?,
            value: row.get(3)?
        })
    }
}

#[derive(Debug)]
pub struct Grapheme {
    pub id: u32,
    pub created_on: DateTime<Utc>,
    pub script_id: u32,
    pub symbol: Box<str>,
}

impl Grapheme {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            script_id: row.get(2)?,
            symbol: row.get(3)?
        })
    }
}

#[derive(Debug)]
pub struct GraphemeReading {
    pub id: u32,
    pub grapheme_id: u32,
    pub language_id: u32,
    pub system: Box<str>,
    pub value: Box<str>,
}

impl GraphemeReading {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            grapheme_id: row.get(1)?,
            language_id: row.get(2)?,
            system: row.get(3)?,
            value: row.get(3)?
        })
    }
}

#[derive(Debug)]
pub struct WordOfTheDay {
    pub id: NaiveDate,
    pub lexeme_id: u32,
    pub language_id: u32
}

impl WordOfTheDay {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            lexeme_id: row.get(1)?,
            language_id: row.get(2)?
        })
    }
}

#[derive(Debug)]
pub struct LanguageFeature {
    pub id: NaiveDate,
    pub created_on: DateTime<Utc>,
    pub language_id: u32,
    pub name: Box<str>,
    pub description: Box<str>,
    pub parent_id: Option<u32>,
}

impl LanguageFeature {
    pub fn from_row(row: &Row<'_>) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            created_on: row.get(1)?,
            language_id: row.get(2)?,
            name: row.get(3)?,
            description: row.get(4)?,
            parent_id: row.get(5)?,
        })
    }
}