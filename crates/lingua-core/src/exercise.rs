use crate::LanguageId;
use alloc::{collections::BTreeSet, string::String, vec::Vec};
use core::fmt;
use serde::{Deserialize, Serialize};

/// Text and optional script/reading metadata; script identifiers use ISO 15924.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text {
    pub value: String,
    pub script: Option<String>,
    pub reading: Option<String>,
}

/// A course's level label. The edition distinguishes revisions of a framework.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurriculumLevel {
    pub framework: String,
    pub edition: Option<String>,
    pub level: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub id: String,
    pub text: Text,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SingleChoiceData")]
pub struct SingleChoice {
    choices: Vec<Choice>,
    correct_choice_id: String,
}

#[derive(Deserialize)]
struct SingleChoiceData {
    choices: Vec<Choice>,
    correct_choice_id: String,
}

impl TryFrom<SingleChoiceData> for SingleChoice {
    type Error = ExerciseError;
    fn try_from(data: SingleChoiceData) -> Result<Self, Self::Error> {
        Self::new(data.choices, data.correct_choice_id)
    }
}

impl SingleChoice {
    pub fn new(choices: Vec<Choice>, correct_choice_id: String) -> Result<Self, ExerciseError> {
        if choices.len() < 2 {
            return Err(ExerciseError::TooFewChoices);
        }
        let mut ids = BTreeSet::new();
        for choice in &choices {
            if choice.id.trim().is_empty() || choice.text.value.trim().is_empty() {
                return Err(ExerciseError::BlankChoice);
            }
            if !ids.insert(&choice.id) {
                return Err(ExerciseError::DuplicateChoiceId);
            }
        }
        if !choices.iter().any(|choice| choice.id == correct_choice_id) {
            return Err(ExerciseError::MissingCorrectChoice);
        }
        Ok(Self {
            choices,
            correct_choice_id,
        })
    }

    pub fn choices(&self) -> &[Choice] {
        &self.choices
    }

    pub fn check_answer(&self, choice_id: &str) -> Result<bool, ExerciseError> {
        if !self.choices.iter().any(|choice| choice.id == choice_id) {
            return Err(ExerciseError::UnknownChoice);
        }
        Ok(choice_id == self.correct_choice_id)
    }
}

/// Interaction format is independent of script and curriculum level.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ExerciseContent {
    SingleChoice(SingleChoice),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub language: LanguageId,
    pub instruction: String,
    pub prompt: Text,
    pub level: Option<CurriculumLevel>,
    pub content: ExerciseContent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseError {
    TooFewChoices,
    BlankChoice,
    DuplicateChoiceId,
    MissingCorrectChoice,
    UnknownChoice,
}

impl fmt::Display for ExerciseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::TooFewChoices => "At least two choices are required.",
            Self::BlankChoice => "Choice identifiers and text must not be blank.",
            Self::DuplicateChoiceId => "Choice identifiers must be unique.",
            Self::MissingCorrectChoice => "The correct choice must exist.",
            Self::UnknownChoice => "The submitted choice does not exist.",
        })
    }
}

impl core::error::Error for ExerciseError {}
