use std::collections::BTreeMap;

use lingua_ai::LlmClient;
use serde::Serialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    exercise_contract as contract,
    types::{
        Concept, Exercise, ExerciseDefinition, LearningEvidence, LearningStage, TeachingGuideline,
    },
};

pub use error::EvaluationError;

mod error;
mod evidence;
mod prompt;
mod response;

pub struct EvaluationRequest<'a> {
    pub user_id: Uuid,
    pub source_language: &'a str,
    pub target_language: &'a str,
    pub exercise: &'a Exercise,
    pub definition: &'a ExerciseDefinition,
    pub answer: Option<&'a Value>,
    pub concepts: &'a [Concept],
    pub learning_stage: &'a LearningStage,
    pub teaching_guidelines: &'a [TeachingGuideline],
}

#[derive(Serialize)]
struct EvaluationPrompt<'a> {
    contract_version: u8,
    source_language: &'a str,
    target_language: &'a str,
    kind: &'a str,
    payload: &'a Value,
    answer: &'a Value,
    verdict_schema: &'a Value,
    concepts: &'a [Concept],
    stage_instructions: &'a str,
    teaching_guidelines: &'a [TeachingGuideline],
}

pub struct EvaluationResult {
    pub verdict: Value,
    pub concept_results: BTreeMap<Uuid, bool>,
    pub evidence: Vec<LearningEvidence>,
}

pub struct ExerciseEvaluator {
    client: LlmClient,
    model: String,
}

impl ExerciseEvaluator {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            client: LlmClient::new(),
            model: model.into(),
        }
    }

    #[tracing::instrument(skip_all, err, fields(stage = "evaluation", exercise_id = %request.exercise.id))]
    pub async fn evaluate(
        &self,
        request: &EvaluationRequest<'_>,
    ) -> Result<EvaluationResult, EvaluationError> {
        let ids = contract::validate_exercise(request.definition, request.exercise)?;
        self.validate_request(request, &ids)?;
        let schema = request
            .definition
            .verdict_schema
            .as_ref()
            .ok_or_else(|| contract::invalid("definition has no verdict schema"))?;
        let validator = contract::compile_schema(schema)?;
        if request.definition.answer_schema.is_none() {
            let verdict = json!({"completed": true, "concept_exposures": ids});
            contract::validate_schema(&validator, &verdict)?;
            let concept_results = BTreeMap::new();
            let evidence =
                evidence::create(request, &verdict, &concept_results, &ids, &self.model)?;
            return Ok(EvaluationResult {
                verdict,
                concept_results,
                evidence,
            });
        }
        let answer = request
            .answer
            .ok_or_else(|| contract::invalid("graded exercise requires an answer"))?;
        let prompt = serde_json::to_string_pretty(&EvaluationPrompt {
            contract_version: 1,
            source_language: request.source_language,
            target_language: request.target_language,
            kind: &request.exercise.kind,
            payload: &request.exercise.payload,
            answer,
            verdict_schema: schema,
            concepts: request.concepts,
            stage_instructions: &request.learning_stage.evaluation_instructions,
            teaching_guidelines: request.teaching_guidelines,
        })?;

        let text = crate::ai_call::generate(
            &self.client,
            &self.model,
            "evaluation",
            prompt::EVALUATION_INSTRUCTIONS,
            &prompt,
        )
        .await
        .map_err(EvaluationError::Generation)?;
        let verdict = contract::parse_response(&text, &validator)?;
        let concept_results = response::validate(&verdict, request.exercise, answer, &ids)?;
        let evidence = evidence::create(request, &verdict, &concept_results, &ids, &self.model)?;
        Ok(EvaluationResult {
            verdict,
            concept_results,
            evidence,
        })
    }

    fn validate_request(
        &self,
        request: &EvaluationRequest<'_>,
        ids: &std::collections::BTreeSet<Uuid>,
    ) -> Result<(), EvaluationError> {
        if self.model.trim().is_empty()
            || request.source_language.split('-').next() != Some("en")
            || request.target_language.trim().is_empty()
        {
            return Err(contract::invalid(
                "evaluation requires a model, English source and a target",
            )
            .into());
        }
        let supplied: std::collections::BTreeSet<_> =
            request.concepts.iter().map(|c| c.id).collect();
        if supplied != *ids
            || supplied.len() != request.concepts.len()
            || request
                .concepts
                .iter()
                .any(|c| c.language_id != request.target_language)
        {
            return Err(contract::invalid(
                "evaluation concepts must match the exercise and target exactly",
            )
            .into());
        }
        match (&request.definition.answer_schema, request.answer) {
            (Some(schema), Some(answer)) => {
                contract::validate_schema(&contract::compile_schema(schema)?, answer)?;
                contract::validate_answer(request.exercise, answer)?;
            }
            (None, None) if request.exercise.kind == "dialogue" => {}
            _ => {
                return Err(contract::invalid(
                    "graded exercises require answers; dialogue has none",
                )
                .into());
            }
        }
        Ok(())
    }
}
