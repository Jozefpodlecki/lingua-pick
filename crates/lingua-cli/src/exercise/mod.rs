mod answer;
mod concepts;
mod contract;
mod error;
mod fields;
mod objective;
mod payload;
mod schema;

pub use answer::validate_answer;
pub use concepts::{concept_ids, parse_id};
pub use contract::validate_exercise;
pub use error::ContractError;
pub use objective::objective_results;
pub use payload::validate_payload;
pub use schema::{compile_schema, parse_response, validate_schema};

pub(crate) type Result<T> = std::result::Result<T, ContractError>;