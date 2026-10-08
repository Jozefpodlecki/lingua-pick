mod lang;
mod lang_exercise;
mod user;
mod stats;
mod exercise;
mod exercise_definition;
mod session;
mod category;

pub use lang::*;
pub use lang_exercise::*;
pub use user::*;
pub use stats::*;
pub use exercise::*;
pub use exercise_definition::*;
pub use session::*;
pub use category::*;

use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("database pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("database error: {0}")]
    Database(#[from] duckdb_neo::error::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}