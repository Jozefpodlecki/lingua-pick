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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageFeature {
    pub id: String,
    pub name: String,
    pub description: String,
    pub group_code: String,
    pub display_order: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageFeatureValue {
    pub feature_id: String,
    pub code: String,
    pub badge_label: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageFeatureAssignment {
    pub language_id: String,
    pub feature_id: String,
    pub value_code: String,
    pub notes: Option<String>,
    pub example: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageFeatureBadge {
    pub language_id: String,
    pub feature_id: String,
    pub feature_name: String,
    pub feature_description: String,
    pub group_code: String,
    pub display_order: i32,
    pub value_code: String,
    pub badge_label: String,
    pub value_description: String,
    pub notes: Option<String>,
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub source_language_id: String,
    pub username: String,
    pub password_hash: String,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: Uuid,
    pub target_language_id: String,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub struggling_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: Uuid,
    pub definition_id: Uuid,
    pub session_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub answered_on: Option<DateTime<Utc>>,
    pub kind: String,
    pub payload: Value,
    pub answer: Option<Value>,
    pub verdict: Option<Value>,
    pub interaction_mode: InteractionMode,
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
    pub last_exercise_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseRequest {
    pub source_language: String,
    pub target_language: String,
    pub user_stats: UserStats,
    pub available_exercises: Vec<ExerciseDefinition>,
    pub available_scripts: Vec<Script>,
    pub learning_stage: LearningStage,
    pub available_topics: Vec<Topic>,
    pub teaching_guidelines: Vec<TeachingGuideline>,
    pub available_skills: Vec<Skill>,
    pub known_concepts: Vec<Concept>,
    pub learner_evidence: Vec<ConceptEvidence>,
    pub recent_exercises: Vec<Exercise>,
}

#[derive(Debug, Clone)]
pub struct GeneratedExercise {
    pub definition_id: Uuid,
    pub kind: String,
    pub payload: Value,
}

#[derive(Debug, Clone)]
pub struct GenerationResult {
    pub exercise: GeneratedExercise,
    pub concepts: Vec<Concept>,
}

impl Exercise {
    pub fn from_generated(session_id: Uuid, generated: GeneratedExercise) -> Self {
        Self {
            id: Uuid::now_v7(),
            definition_id: generated.definition_id,
            session_id,
            created_on: Utc::now(),
            answered_on: None,
            kind: generated.kind,
            payload: generated.payload,
            answer: None,
            verdict: None,
            interaction_mode: InteractionMode::SingleTurn,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceMode {
    Recognition,
    Recall,
    Production,
    Comprehension,
    Exposure,
    Practice,
}
impl EvidenceMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recognition => "recognition",
            Self::Recall => "recall",
            Self::Production => "production",
            Self::Comprehension => "comprehension",
            Self::Exposure => "exposure",
            Self::Practice => "practice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionMode {
    SingleTurn,
    MultiTurn,
}
impl InteractionMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleTurn => "single_turn",
            Self::MultiTurn => "multi_turn",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnRole {
    System,
    Assistant,
    User,
}
impl TurnRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Assistant => "assistant",
            Self::User => "user",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExerciseCategory {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Concept {
    pub id: Uuid,
    pub language_id: String,
    pub skill_id: String,
    pub topic_id: String,
    pub code: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExerciseConcept {
    pub exercise_id: Uuid,
    pub concept_id: Uuid,
    pub is_primary: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExerciseTurn {
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub turn_number: u16,
    pub role: TurnRole,
    pub content: String,
    pub evaluation: Option<Value>,
    pub created_on: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserConcept {
    pub user_id: Uuid,
    pub target_language_id: String,
    pub concept_id: Uuid,
    pub attempts: i32,
    pub correct: i32,
    pub mastery: Option<f64>,
    pub confidence: Option<f64>,
    pub last_seen: Option<DateTime<Utc>>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningStage {
    pub code: String,
    pub sequence: u16,
    pub name: String,
    pub generation_instructions: String,
    pub evaluation_instructions: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageSkill {
    pub stage_code: String,
    pub skill_id: String,
    pub evidence_mode: EvidenceMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageExercise {
    pub stage_code: String,
    pub definition_id: Uuid,
    pub selection_instructions: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicStage {
    pub topic_id: String,
    pub stage_code: String,
    pub generation_instructions: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeachingGuideline {
    pub code: String,
    pub category: String,
    pub generation_instructions: String,
    pub evaluation_instructions: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningEvidence {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_language_id: String,
    pub exercise_id: Uuid,
    pub concept_id: Uuid,
    pub event_key: String,
    pub evidence_mode: EvidenceMode,
    pub correct: Option<bool>,
    pub assisted: bool,
    pub occurred_on: DateTime<Utc>,
    pub evaluator: String,
    pub policy_version: String,
    pub detail: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptEvidence {
    pub user_id: Uuid,
    pub target_language_id: String,
    pub concept_id: Uuid,
    pub evidence_mode: EvidenceMode,
    pub encounters: u64,
    pub assessed_attempts: u64,
    pub correct_attempts: u64,
    pub assessed_sessions: u64,
    pub last_seen: DateTime<Utc>,
    pub accuracy: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageExercise {
    pub language_id: String,
    pub definition_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageScript {
    pub language_id: String,
    pub script_id: String,
}
