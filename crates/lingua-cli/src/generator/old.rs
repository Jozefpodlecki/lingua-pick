use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use lingua_ai::LlmClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::{prompt::EXERCISE_INSTRUCTIONS, types::ExerciseRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseDefinition {
    pub kind: String,
    pub description: String,
    pub schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub kind: String,
    pub data: Value,
}

pub struct ExerciseGenerator {
    client: LlmClient,
    model: String,
}

#[derive(Debug, Error)]
pub enum ExerciseGenerationError {
    #[error("LLM generation failed: {0}")]
    Generation(String),

    #[error("could not serialize exercise definitions: {0}")]
    Definition(String),

    #[error("no exercise definitions available")]
    NoExerciseDefinitions,

    #[error("LLM returned invalid JSON: {error}\n\nResponse:\n{response}")]
    InvalidResponse {
        error: String,
        response: String,
    },

    #[error("LLM returned unknown exercise kind '{0}'")]
    UnknownKind(String),

    #[error("LLM generated an invalid exercise: {0}")]
    InvalidExercise(String),
}

impl ExerciseDefinition {
    pub fn new(
        kind: impl Into<String>,
        description: impl Into<String>,
        schema: Value,
    ) -> Self {
        Self {
            kind: kind.into(),
            description: description.into(),
            schema,
        }
    }
}

impl ExerciseGenerator {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            client: LlmClient::new(),
            model: model.into(),
        }
    }

    pub async fn generate(
        &self,
        request: &ExerciseRequest,
    ) -> Result<Exercise, ExerciseGenerationError> {
        let prompt = build_prompt(request)?;

        let response = self
            .client
            .generate(
                &self.model,
                EXERCISE_INSTRUCTIONS,
                &prompt,
            )
            .await
            .map_err(|error| {
                ExerciseGenerationError::Generation(
                    error.to_string(),
                )
            })?;

        let exercise: Exercise =
            serde_json::from_str(&response).map_err(|error| {
                ExerciseGenerationError::InvalidResponse {
                    error: error.to_string(),
                    response,
                }
            })?;

        validate_exercise(request, &exercise)?;

        Ok(exercise)
    }
}

fn build_prompt(
    request: &ExerciseRequest,
) -> Result<String, ExerciseGenerationError> {
    if request.available_exercises.is_empty() {
        return Err(
            ExerciseGenerationError::NoExerciseDefinitions,
        );
    }

    let definitions = serde_json::to_string_pretty(
        &request.available_exercises,
    )
    .map_err(|error| {
        ExerciseGenerationError::Definition(
            error.to_string(),
        )
    })?;

    // let categories =
    //     if request.struggling_categories.is_empty() {
    //         String::from("None specified")
    //     } else {
    //         request.struggling_categories.join(", ")
    //     };

    Ok(format!(
        r#"
Generate one language-learning exercise.

SOURCE LANGUAGE
{source_language}

TARGET LANGUAGE
{target_language}

AREAS THAT NEED REINFORCEMENT
{categories}

AVAILABLE EXERCISE TYPES
{definitions}

Choose exactly one exercise type from AVAILABLE EXERCISE TYPES.

Return exactly one JSON object with this structure:

{{
    "kind": "<exact kind from the selected exercise definition>",
    "data": <data conforming exactly to the selected exercise schema>
}}
"#,
        source_language = request.source_language,
        target_language = request.target_language,
        categories = categories,
        definitions = definitions,
    ))
}

fn validate_exercise(
    request: &ExerciseRequest,
    exercise: &Exercise,
) -> Result<(), ExerciseGenerationError> {
    let definition = request
        .available_exercises
        .iter()
        .find(|definition| {
            definition.kind == exercise.kind
        })
        .ok_or_else(|| {
            ExerciseGenerationError::UnknownKind(
                exercise.kind.clone(),
            )
        })?;

    if exercise.data.is_null() {
        return Err(
            ExerciseGenerationError::InvalidExercise(
                "exercise data cannot be null".into(),
            ),
        );
    }

    Ok(())
}