use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error(transparent)]
    Seed(#[from] crate::seed::SeedError),
    #[error(transparent)]
    Store(#[from] crate::store::StoreError),
    #[error(transparent)]
    Generation(#[from] crate::generator::ExerciseGenerationError),
    #[error(transparent)]
    Simulation(#[from] crate::simulator::SimulationError),
    #[error(transparent)]
    Evaluation(#[from] crate::evaluator::EvaluationError),
    #[error(transparent)]
    Contract(#[from] crate::exercise_contract::ContractError),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    #[error("invalid runner state: {0}")]
    InvalidState(&'static str),
}
