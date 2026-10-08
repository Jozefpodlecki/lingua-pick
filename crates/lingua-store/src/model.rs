use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentSource {
    Authored,
    Sample,
    Generated,
}

impl ContentSource {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Sample => "sample",
            Self::Generated => "generated",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "authored" => Some(Self::Authored),
            "sample" => Some(Self::Sample),
            "generated" => Some(Self::Generated),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    InProgress,
    Completed,
    Abandoned,
    Failed,
}

impl SessionStatus {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "in_progress" => Some(Self::InProgress),
            "completed" => Some(Self::Completed),
            "abandoned" => Some(Self::Abandoned),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewSession {
    pub id: SessionId,
    pub target_language_id: String,
    pub content_source: ContentSource,
    pub total_exercises: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewSessionExercise {
    pub position: u16,
    pub exercise_id: String,
    pub exercise_kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExerciseResult {
    pub position: u16,
    pub answer: Value,
    pub correct: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: SessionId,
    pub target_language_id: String,
    pub content_source: ContentSource,
    pub status: SessionStatus,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_exercises: u16,
    pub answered_exercises: u16,
    pub correct_exercises: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionExerciseRecord {
    pub session_id: SessionId,
    pub position: u16,
    pub exercise_id: String,
    pub exercise_kind: String,
    pub payload: Value,
    pub answer: Option<Value>,
    pub correct: Option<bool>,
    pub presented_at: DateTime<Utc>,
    pub answered_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetStats {
    pub target_language_id: String,
    pub sessions_started: u64,
    pub sessions_completed: u64,
    pub answers_recorded: u64,
    pub graded_answers: u64,
    pub correct_answers: u64,
    pub accuracy: Option<f64>,
}
