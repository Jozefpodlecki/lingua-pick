use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvaluationError {
    #[error("invalid evaluation request or response: {0}")]
    Contract(#[from] crate::exercise_contract::ContractError),
    #[error("could not serialize evaluation request: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("answer evaluation failed: {0}")]
    Generation(String),
}
