use thiserror::Error;

use crate::{evaluator::EvaluationError, exercise::ContractError, generator::ExerciseGenerationError, seed::SeedError, simulator::SimulationError, store::StoreError, tools::ToolError};

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error(transparent)]
    Seed(#[from] SeedError),

    #[error(transparent)]
    Tool(#[from] ToolError),
    
    #[error(transparent)]
    Store(#[from] StoreError),
    
    #[error(transparent)]
    Generation(#[from] ExerciseGenerationError),
    
    #[error(transparent)]
    Simulation(#[from] SimulationError),
    
    #[error(transparent)]
    Evaluation(#[from] EvaluationError),
    
    #[error(transparent)]
    Contract(#[from] ContractError),
    
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    
    #[error("invalid runner state: {0}")]
    InvalidState(&'static str),
}
