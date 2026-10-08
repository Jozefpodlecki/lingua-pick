use thiserror::Error;

use crate::conversation::ConversationError;

#[derive(Debug, Error)]
pub enum ExerciseGenerationError {
    #[error("invalid generation request: {0}")]
    InvalidRequest(String),

    #[error("invalid schema for '{kind}': {message}")]
    InvalidSchema {
        kind: String,
        message: String,
    },

    #[error("could not serialize generation request: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error(transparent)]
    Conversation(#[from] ConversationError),

    #[error(transparent)]
    Tool(#[from] crate::tools::ToolError),

    #[error("model returned invalid JSON: {message}\nResponse: {response}")]
    InvalidResponse {
        message: String,
        response: String,
    },

    #[error("model returned unavailable exercise kind '{0}'")]
    UnknownKind(String),

    #[error("generated exercise is invalid: {0}")]
    InvalidExercise(String),
}