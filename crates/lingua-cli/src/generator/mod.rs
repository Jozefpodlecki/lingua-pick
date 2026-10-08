use lingua_ai::LlmClient;
use serde::Serialize;
use uuid::Uuid;

use self::prompt::EXERCISE_INSTRUCTIONS;
use crate::{
    tools::GenerationTools,
    types::{ExerciseRequest, GenerationResult},
};

pub use error::ExerciseGenerationError;

mod error;
mod prompt;
mod response;
mod validation;

const CONCEPT_SLOT_COUNT: usize = 10;

#[derive(Serialize)]
struct GenerationPrompt<'a> {
    contract_version: u8,
    request: &'a ExerciseRequest,
    new_concept_slots: &'a [Uuid],
}

pub struct ExerciseGenerator {
    client: LlmClient,
    model: String,
}

impl ExerciseGenerator {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            client: LlmClient::new(),
            model: model.into(),
        }
    }

    #[tracing::instrument(skip_all, err, fields(stage = "generation"))]
    pub async fn generate(
        &self,
        request: &ExerciseRequest,
        tools: &mut GenerationTools,
    ) -> Result<GenerationResult, ExerciseGenerationError> {
        self.validate_request(request)?;
        let validators = compile_schemas(request)?;
        let slots: Vec<Uuid> = (0..CONCEPT_SLOT_COUNT).map(|_| Uuid::now_v7()).collect();
        let prompt = build_prompt(request, &slots)?;

        let response = self.request_exercise(&prompt, tools).await?;
        let mut validation_request = request.clone();
        validation_request.known_concepts = tools.retrieved_concepts();
        let result = response::decode(response, &validation_request, &validators, &slots)?;
        tools
            .validate_new_codes(&result)
            .await
            .map_err(ExerciseGenerationError::InvalidExercise)?;
        Ok(result)
    }

    fn validate_request(&self, request: &ExerciseRequest) -> Result<(), ExerciseGenerationError> {
        validation::validate_request(request)?;
        if self.model.trim().is_empty() {
            return Err(ExerciseGenerationError::InvalidRequest(
                "model identifier cannot be blank".into(),
            ));
        }

        Ok(())
    }

    async fn request_exercise(
        &self,
        prompt: &str,
        tools: &mut GenerationTools,
    ) -> Result<String, ExerciseGenerationError> {
        crate::ai_call::conversation::generate(
            &self.client,
            &self.model,
            EXERCISE_INSTRUCTIONS,
            prompt,
            tools,
        )
        .await
        .map_err(ExerciseGenerationError::Generation)
    }
}

fn compile_schemas(
    request: &ExerciseRequest,
) -> Result<Vec<jsonschema::Validator>, ExerciseGenerationError> {
    request
        .available_exercises
        .iter()
        .map(|definition| {
            jsonschema::draft202012::options()
                .should_validate_formats(true)
                .build(&definition.schema)
                .map_err(|error| ExerciseGenerationError::InvalidSchema {
                    kind: definition.kind.clone(),
                    message: error.to_string(),
                })
        })
        .collect()
}

fn build_prompt(
    request: &ExerciseRequest,
    slots: &[Uuid],
) -> Result<String, ExerciseGenerationError> {
    let mut initial_request = request.clone();
    initial_request.known_concepts.clear();
    initial_request.learner_evidence.clear();
    initial_request.recent_exercises.clear();
    Ok(serde_json::to_string_pretty(&GenerationPrompt {
        contract_version: 1,
        request: &initial_request,
        new_concept_slots: slots,
    })?)
}
