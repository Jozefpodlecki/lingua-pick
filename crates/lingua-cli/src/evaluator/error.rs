use thiserror::Error;
use uuid::Uuid;

use crate::exercise::ContractError;

#[derive(Debug, Error)]
pub enum EvaluationError {
    
    #[error("invalid evaluation request or response: {0}")]
    Contract(#[from] ContractError),
    
    #[error("could not serialize evaluation request: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("answer evaluation failed: {0}")]
    Generation(String),
    
    #[error("evaluation model cannot be blank")]
    MissingModel,

    #[error("unsupported source language: {0}")]
    UnsupportedSourceLanguage(String),

    #[error("target language cannot be blank")]
    MissingTargetLanguage,

    #[error("exercise definition has no verdict schema")]
    MissingVerdictSchema,

    #[error("graded exercise requires an answer")]
    MissingAnswer,

    #[error("ungraded exercise must not have an answer")]
    UnexpectedAnswer,

    #[error("ungraded exercise kind is unsupported: {0}")]
    UnsupportedUngradedKind(String),

    #[error("evaluation contains duplicate concepts")]
    DuplicateConcepts,

    #[error("evaluation concepts do not match exercise concepts")]
    ConceptMismatch,

    #[error(
        "concept {concept_id} has language '{actual}', expected '{expected}'"
    )]
    ConceptLanguageMismatch {
        concept_id: Uuid,
        expected: String,
        actual: String,
    },

    #[error("exercise kind '{0}' has no evidence policy")]
    UnsupportedEvidenceKind(String),

    #[error("verdict is missing concept_results")]
    MissingConceptResults,

    #[error("concept result must contain a boolean 'correct' field")]
    InvalidConceptResult,

    #[error("duplicate verdict result for concept {id}")]
    DuplicateConceptResult { id: Uuid },

    #[error(
        "verdict concept coverage mismatch: missing {missing:?}, unexpected {unexpected:?}"
    )]
    ConceptResultsMismatch {
        missing: Vec<Uuid>,
        unexpected: Vec<Uuid>,
    },

    #[error("verdict requires nonblank feedback")]
    MissingFeedback,

    #[error(
        "objective result mismatch for concept {concept_id}: expected {expected}, got {actual}"
    )]
    ObjectiveResultMismatch {
        concept_id: Uuid,
        expected: bool,
        actual: bool,
    },
}
