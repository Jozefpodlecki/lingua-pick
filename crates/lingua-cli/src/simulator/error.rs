use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SimulationError {
    
    #[error("invalid simulation configuration: {0}")]
    Configuration(String),

    #[error("invalid simulation request or response: {0}")]
    Contract(#[from] crate::exercise::ContractError),

    #[error("could not serialize simulation request: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("could not sample simulation outcome: {0}")]
    Random(String),

    #[error("learner simulation failed: {0}")]
    Generation(String),

    #[error("simulation model cannot be blank")]
    MissingModel,

    #[error("unsupported source language: {0}")]
    UnsupportedSourceLanguage(String),

    #[error("target language cannot be blank")]
    MissingTargetLanguage,

    #[error("simulation evidence belongs to user {actual}, expected {expected}")]
    EvidenceUserMismatch {
        expected: Uuid,
        actual: Uuid,
    },

    #[error("simulation evidence has language '{actual}', expected '{expected}'")]
    EvidenceLanguageMismatch {
        expected: String,
        actual: String,
    },

    #[error("ungraded simulation is unsupported for exercise kind '{0}'")]
    UnsupportedUngradedKind(String),

    #[error("exercise kind cannot be blank")]
    EmptyExerciseKind,

    #[error("mistake percentage must be between 0 and 100, got {percentage}")]
    InvalidMistakePercentage { percentage: u8 },

    #[error("too many matching concepts")]
    TooManyMatchingConcepts,

    #[error("matching requires at least two concepts to simulate mistakes, got {count}")]
    InsufficientMatchingConcepts { count: usize },

    #[error("simulated answer does not match the sampled mistake plan")]
    MistakePlanMismatch,

    #[error("random range must be greater than zero")]
    InvalidRandomRange,
}
