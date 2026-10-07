use crate::{Exercise, ExerciseContent, ExerciseError, LanguageId};
use alloc::{collections::BTreeSet, vec::Vec};
use core::fmt;

/// An ordered, single-target session. Each valid answer advances one exercise.
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
        if exercises.is_empty() {
            return Err(SessionError::EmptySession);
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
    EmptySession,
    LanguageMismatch,
    BlankExerciseId,
    DuplicateExerciseId,
    Complete,
    InvalidAnswer(ExerciseError),
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAnswer(error) => error.fmt(f),
            error => f.write_str(match error {
                Self::BlankLanguage => "The target language must not be blank.",
                Self::EmptySession => "A session must contain exercises.",
                Self::LanguageMismatch => "Every exercise must belong to the session target.",
                Self::BlankExerciseId => "Exercise identifiers must not be blank.",
                Self::DuplicateExerciseId => {
                    "Exercise identifiers must be unique within a session."
                }
                Self::Complete => "The session is already complete.",
                Self::InvalidAnswer(_) => unreachable!(),
            }),
        }
    }
}

impl core::error::Error for SessionError {}
