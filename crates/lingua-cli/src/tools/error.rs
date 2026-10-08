
use thiserror::Error;

use crate::store::StoreError;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("invalid tool arguments: {0}")]
    Arguments(#[from] serde_json::Error),

    #[error("store operation failed: {0}")]
    Store(#[from] StoreError),

    #[error("invalid tool definition: {0}")]
    Definition(String),

    #[error("invalid tool schema for '{tool}': {message}")]
    Schema { tool: String, message: String },

    #[error("unknown tool: {0}")]
    UnknownTool(String),

    #[error("invalid tool call")]
    InvalidCall,

    #[error("provider-executed tool call is not allowed")]
    ProviderExecuted,

    #[error("invalid arguments for '{tool}': {message}")]
    Validation { tool: String, message: String },

    #[error("tool execution task failed: {0}")]
    Task(#[from] tokio::task::JoinError),

    #[error("tool result exceeds {limit} bytes")]
    ResultTooLarge { limit: usize },

    #[error("new concept code already exists: {0}")]
    DuplicateConceptCode(String),

    #[error("{0}")]
    Invalid(String),
}
