use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub region: String,
    pub native_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub source_language_id: String,
    pub username: String,
    pub password_hash: String,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: Uuid,
    pub target_language_id: String,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub struggling_categories: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: Uuid,
    pub session_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub answered_on: Option<DateTime<Utc>>,
    pub kind: String,
    pub payload: Value,
    pub answer: Option<Value>,
    pub verdict: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseDefinition {
    pub id: Uuid,
    pub kind: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub instructions: String,
    pub schema: Value,
    pub answer_schema: Option<Value>,
    pub verdict_schema: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_language_id: String,
    pub created_on: DateTime<Utc>,
    pub exercise_count: u16,
    pub max_exercise_count: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseRequest {
    pub source_language: String,
    pub target_language: String,
    pub user_stats: UserStats,
    pub available_exercises: Vec<ExerciseDefinition>,
}
