use crate::{Exercise, ExerciseContent, ExerciseError, LanguageId};
use alloc::{collections::BTreeSet, vec::Vec};
use core::fmt;

pub const SESSION_EXERCISE_COUNT: usize = 10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    language: LanguageId,
    exercises: Vec<Exercise>,
    answers: Vec<AnswerOutcome>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnswerOutcome {
    pub exercise_id: alloc::string::String,
    pub choice_id: alloc::string::String,
    pub correct: bool,
}

impl Session {
    pub fn new(language: LanguageId, exercises: Vec<Exercise>) -> Result<Self, SessionError> {
        if language.0.trim().is_empty() {
            return Err(SessionError::BlankLanguage);
        }

        if exercises.len() != SESSION_EXERCISE_COUNT {
            return Err(SessionError::ExerciseCount {
                expected: SESSION_EXERCISE_COUNT,
                actual: exercises.len(),
            });
        }

        let mut ids = BTreeSet::new();

        for exercise in &exercises {
            if exercise.language != language {
                return Err(SessionError::LanguageMismatch);
            }
            if exercise.id.trim().is_empty() {
                return Err(SessionError::BlankExerciseId);
            }
            if !ids.insert(&exercise.id) {
                return Err(SessionError::DuplicateExerciseId);
            }
        }
        Ok(Self {
            language,
            exercises,
            answers: Vec::new(),
        })
    }

    pub fn language(&self) -> &LanguageId {
        &self.language
    }

    pub fn current(&self) -> Option<&Exercise> {
        self.exercises.get(self.answers.len())
    }

    pub fn answers(&self) -> &[AnswerOutcome] {
        &self.answers
    }

    pub fn total(&self) -> usize {
        self.exercises.len()
    }

    pub fn is_complete(&self) -> bool {
        self.current().is_none()
    }

    pub fn correct_count(&self) -> usize {
        self.answers.iter().filter(|answer| answer.correct).count()
    }

    pub fn answer(&mut self, choice_id: &str) -> Result<AnswerOutcome, SessionError> {
        let exercise = self.current().ok_or(SessionError::Complete)?;
        let correct = match &exercise.content {
            ExerciseContent::SingleChoice(content) => content
                .check_answer(choice_id)
                .map_err(SessionError::InvalidAnswer)?,
        };
        let outcome = AnswerOutcome {
            exercise_id: exercise.id.clone(),
            choice_id: choice_id.into(),
            correct,
        };
        self.answers.push(outcome.clone());
        Ok(outcome)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionError {
    BlankLanguage,
    ExerciseCount { expected: usize, actual: usize },
    LanguageMismatch,
    BlankExerciseId,
    DuplicateExerciseId,
    Complete,
    InvalidAnswer(ExerciseError),
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankLanguage => f.write_str("The target language must not be blank."),
            Self::ExerciseCount { expected, actual } => {
                write!(
                    f,
                    "A session must contain exactly {expected} exercises; received {actual}."
                )
            }
            Self::LanguageMismatch => {
                f.write_str("Every exercise must belong to the session target.")
            }
            Self::BlankExerciseId => f.write_str("Exercise identifiers must not be blank."),
            Self::DuplicateExerciseId => {
                f.write_str("Exercise identifiers must be unique within a session.")
            }
            Self::Complete => f.write_str("The session is already complete."),
            Self::InvalidAnswer(error) => error.fmt(f),
        }
    }
}

impl core::error::Error for SessionError {}
