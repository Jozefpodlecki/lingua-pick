
use lingua_ai::LlmClient;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    exercise as contract,
    types::{ConceptEvidence, Exercise, ExerciseDefinition},
};

pub use distribution::{MistakeDistribution, SimulationPlan};
pub use error::SimulationError;

mod distribution;
mod error;
mod prompt;

pub struct SimulationRequest<'a> {
    pub user_id: Uuid,
    pub source_language: &'a str,
    pub target_language: &'a str,
    pub exercise: &'a Exercise,
    pub definition: &'a ExerciseDefinition,
    pub learner_evidence: &'a [ConceptEvidence],
}

#[derive(Serialize)]
struct SimulationPrompt<'a> {
    contract_version: u8,
    source_language: &'a str,
    target_language: &'a str,
    kind: &'a str,
    payload: &'a Value,
    answer_schema: &'a Value,
    learner_evidence: &'a [ConceptEvidence],
    plan: &'a SimulationPlan,
}

pub struct SimulationResult {
    pub answer: Option<Value>,
    pub plan: SimulationPlan,
}

pub struct ExerciseSimulator {
    client: LlmClient,
    model: String,
    distribution: MistakeDistribution,
}

impl ExerciseSimulator {
    pub fn new(
        model: impl Into<String>,
        distribution: MistakeDistribution,
    ) -> Self {
        Self {
            client: LlmClient::new(),
            model: model.into(),
            distribution,
        }
    }

    #[tracing::instrument(
        skip_all,
        err,
        fields(
            stage = "simulation",
            exercise_id = %request.exercise.id
        )
    )]
    pub async fn answer(
        &self,
        request: &SimulationRequest<'_>,
    ) -> Result<SimulationResult, SimulationError> {
        self.validate_request(request)?;

        let plan = self.distribution.plan(
            request.exercise,
            request.definition.answer_schema.is_some(),
        )?;

        tracing::debug!(
            plan = %serde_json::to_string(&plan)?,
            "Simulation outcome sampled"
        );

        let Some(schema) = request.definition.answer_schema.as_ref() else {
            return Ok(SimulationResult {
                answer: None,
                plan,
            });
        };

        let validator = contract::compile_schema(schema)?;

        let prompt = serde_json::to_string_pretty(&SimulationPrompt {
            contract_version: 1,
            source_language: request.source_language,
            target_language: request.target_language,
            kind: &request.exercise.kind,
            payload: &request.exercise.payload,
            answer_schema: schema,
            learner_evidence: request.learner_evidence,
            plan: &plan,
        })?;

        let response = crate::ai_call::generate(
            &self.client,
            &self.model,
            "simulation",
            prompt::SIMULATION_INSTRUCTIONS,
            &prompt,
        )
        .await
        .map_err(SimulationError::Generation)?;

        let answer = contract::parse_response(
            &response,
            &validator,
        )?;

        contract::validate_answer(request.exercise, &answer)?;

        if let Some(results) =
            contract::objective_results(request.exercise, &answer)?
        {
            plan.validate_results(&results)?;
        }

        Ok(SimulationResult {
            answer: Some(answer),
            plan,
        })
    }

    fn validate_request(
        &self,
        request: &SimulationRequest<'_>,
    ) -> Result<(), SimulationError> {
        contract::validate_exercise(
            request.definition,
            request.exercise,
        )?;

        if self.model.trim().is_empty() {
            return Err(SimulationError::MissingModel);
        }

        if request.source_language.split('-').next() != Some("en") {
            return Err(SimulationError::UnsupportedSourceLanguage(
                request.source_language.into(),
            ));
        }

        if request.target_language.trim().is_empty() {
            return Err(SimulationError::MissingTargetLanguage);
        }

        if let Some(evidence) = request
            .learner_evidence
            .iter()
            .find(|evidence| evidence.user_id != request.user_id)
        {
            return Err(SimulationError::EvidenceUserMismatch {
                expected: request.user_id,
                actual: evidence.user_id,
            });
        }

        if let Some(evidence) = request
            .learner_evidence
            .iter()
            .find(|evidence| {
                evidence.target_language_id != request.target_language
            })
        {
            return Err(SimulationError::EvidenceLanguageMismatch {
                expected: request.target_language.into(),
                actual: evidence.target_language_id.clone(),
            });
        }

        if request.definition.answer_schema.is_none()
            && request.exercise.kind != "dialogue"
        {
            return Err(SimulationError::UnsupportedUngradedKind(
                request.exercise.kind.clone(),
            ));
        }

        Ok(())
    }
}
