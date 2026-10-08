use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid JSON schema: {message}")]
    InvalidSchema { message: String },

    #[error("schema validation failed: {}", .errors.join("; "))]
    SchemaViolation { errors: Vec<String> },

    #[error("response exceeds {limit} bytes")]
    ResponseTooLarge { limit: usize },

    #[error("exercise does not match definition")]
    DefinitionMismatch,

    #[error("exercise must be pending and single-turn")]
    UnsupportedExerciseState,

    #[error("exercise must reference at least one concept")]
    MissingConcepts,

    #[error("field '{field}' is required")]
    MissingField { field: String },

    #[error("field '{field}' must be {expected}")]
    InvalidFieldType {
        field: String,
        expected: &'static str,
    },

    #[error("field '{field}' contains an invalid UUID")]
    InvalidUuid { field: String },

    #[error("field '{field}' contains a blank string")]
    BlankString { field: String },

    #[error("field '{field}' exceeds {max} characters")]
    StringTooLong { field: String, max: usize },

    #[error("duplicate value at {path} (conflicts with {conflicting_path}, rule: {rule}): {value}")]
    DuplicateValue {
        field: String,
        value: String,
        path: String,
        conflicting_path: String,
        rule: String,
    },

    #[error("unknown reference in field '{field}': {value}")]
    UnknownReference { field: String, value: String },

    #[error("duplicate reference in field '{field}': {value}")]
    DuplicateReference { field: String, value: String },

    #[error("answer must cover all required items")]
    IncompleteAnswer,

    #[error("answer must establish a one-to-one matching")]
    InvalidMatching,

    #[error("invalid token sequence")]
    InvalidTokenSequence,

    #[error("invalid accepted token order")]
    InvalidAcceptedOrder,

    #[error("concept {id} has conflicting results")]
    ConflictingConceptResult { id: Uuid },
}
