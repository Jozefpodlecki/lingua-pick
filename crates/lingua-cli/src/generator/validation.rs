use std::collections::BTreeSet;

use serde_json::Value;
use uuid::Uuid;

use super::ExerciseGenerationError;
use crate::{exercise, types::ExerciseRequest};

pub(super) fn validate_request(request: &ExerciseRequest) -> Result<(), ExerciseGenerationError> {
    let fail = |message: &str| ExerciseGenerationError::InvalidRequest(message.into());
    if request.source_language.split('-').next() != Some("en")
        || request.target_language.trim().is_empty()
    {
        return Err(fail("English source and a nonblank target are required"));
    }
    if request.user_stats.target_language_id != request.target_language
        || request.available_exercises.is_empty()
    {
        return Err(fail(
            "learner state must match the target and at least one exercise definition is required",
        ));
    }
    let mut kinds = BTreeSet::new();
    let mut definitions = BTreeSet::new();
    for definition in &request.available_exercises {
        if definition.kind.trim().is_empty()
            || !kinds.insert(&definition.kind)
            || !definitions.insert(definition.id)
            || !definition.schema.is_object()
            || definition.instructions.trim().is_empty()
        {
            return Err(fail(
                "exercise definitions require distinct IDs and kinds, object schemas and instructions",
            ));
        }
        if request.learning_stage.code == "words" && definition.kind != "match_words" {
            return Err(fail(
                "the introductory words stage only permits match_words",
            ));
        }
    }
    if request
        .learning_stage
        .generation_instructions
        .trim()
        .is_empty()
        || request.available_skills.is_empty()
        || request.available_topics.is_empty()
    {
        return Err(fail(
            "teaching instructions, skills and topics are required",
        ));
    }
    let mut ids = BTreeSet::new();
    let mut codes = BTreeSet::new();
    for concept in &request.known_concepts {
        if concept.language_id != request.target_language
            || !ids.insert(concept.id)
            || !codes.insert(&concept.code)
        {
            return Err(fail(
                "known concepts must have unique IDs and codes and belong to the target",
            ));
        }
    }
    for evidence in &request.learner_evidence {
        if evidence.user_id != request.user_stats.user_id
            || evidence.target_language_id != request.target_language
            || !ids.contains(&evidence.concept_id)
        {
            return Err(fail(
                "learner evidence must match the learner, target and known concepts",
            ));
        }
    }
    for exercise in &request.recent_exercises {
        if concept_ids(&exercise.payload)?
            .iter()
            .any(|id| !ids.contains(id))
        {
            return Err(fail(
                "recent exercises reference concepts outside the target context",
            ));
        }
    }
    Ok(())
}

pub(super) fn concept_ids(value: &Value) -> Result<BTreeSet<Uuid>, ExerciseGenerationError> {
    exercise::concept_ids(value)
        .map_err(|error| ExerciseGenerationError::InvalidExercise(error.to_string()))
}

pub(super) fn validate_payload(kind: &str, data: &Value) -> Result<(), ExerciseGenerationError> {
    exercise::validate_payload(kind, data)
        .map_err(|error| ExerciseGenerationError::InvalidExercise(error.to_string()))
}
