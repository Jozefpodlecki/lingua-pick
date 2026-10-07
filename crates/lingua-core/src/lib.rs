#![no_std]

//! Browser-independent learning models. The application supplies an allocator.
extern crate alloc;

pub mod exercise;
pub mod language;
pub mod session;

pub use exercise::{
    Choice, CurriculumLevel, Exercise, ExerciseContent, ExerciseError, SingleChoice, Text,
};
pub use language::LanguageId;
pub use session::{AnswerOutcome, Session, SessionError};
