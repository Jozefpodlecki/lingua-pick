mod concept;
mod exercise;
mod exercise_category;
mod exercise_concept;
mod exercise_definition;
mod exercise_turn;
mod lang;
mod lang_exercise;
mod language_script;
mod learning_evidence;
mod learning_stage;
mod query;
mod script;
mod session;
mod skill;
mod stats;
mod teaching_guideline;
mod topic;
mod user;
mod user_concept;

pub use concept::*;
pub use exercise::*;
pub use exercise_category::*;
pub use exercise_concept::*;
pub use exercise_definition::*;
pub use exercise_turn::*;
pub use lang::*;
pub use lang_exercise::*;
pub use language_script::*;
pub use learning_evidence::*;
pub use learning_stage::*;
pub use script::*;
pub use session::*;
pub use skill::*;
pub use stats::*;
pub use teaching_guideline::*;
pub use topic::*;
pub use user::*;
pub use user_concept::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("database pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("database error: {0}")]
    Database(#[from] duckdb_neo::error::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("record not found")]
    NotFound,

    #[error("invalid stored record: {0}")]
    InvalidRecord(&'static str),

    #[error("invalid store input: {0}")]
    InvalidInput(&'static str),

    #[error("file access error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests;
