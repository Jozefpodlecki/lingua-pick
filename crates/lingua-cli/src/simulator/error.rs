use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("invalid simulation configuration: {0}")]
    Configuration(String),
    #[error("invalid simulation request or response: {0}")]
    Contract(#[from] crate::exercise_contract::ContractError),
    #[error("could not serialize simulation request: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("could not sample simulation outcome: {0}")]
    Random(String),
    #[error("learner simulation failed: {0}")]
    Generation(String),
}
