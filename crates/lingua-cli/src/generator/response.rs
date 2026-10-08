use std::collections::BTreeSet;

use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use super::{ExerciseGenerationError, validation};
use crate::types::{Concept, ExerciseRequest, GeneratedExercise, GenerationResult};

const MAX_RESPONSE_BYTES: usize = 128 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelExercise {
    kind: String,
    data: Value,
    concepts: Vec<ModelConcept>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelConcept {
    id: Uuid,
    code: String,
    name: String,
    description: String,
    skill_id: String,
}

pub(super) fn decode(
    response: String,
    request: &ExerciseRequest,
    validators: &[jsonschema::Validator],
    slots: &[Uuid],
) -> Result<GenerationResult, ExerciseGenerationError> {
    let candidate = parse_response(response)?;
    let definition_id = validate_exercise(&candidate, request, validators)?;
    let concepts = resolve_concepts(request, slots, &candidate.data, candidate.concepts)?;

    Ok(GenerationResult {
        exercise: GeneratedExercise {
            definition_id,
            kind: candidate.kind,
            payload: candidate.data,
        },
        concepts,
    })
}

fn parse_response(response: String) -> Result<ModelExercise, ExerciseGenerationError> {
    if response.len() > MAX_RESPONSE_BYTES {
        return Err(ExerciseGenerationError::InvalidExercise(
            "response exceeds 128 KiB".into(),
        ));
    }
    let candidate: ModelExercise = serde_json::from_str(&response).map_err(|error| {
        ExerciseGenerationError::InvalidResponse {
            message: error.to_string(),
            response,
        }
    })?;
    Ok(candidate)
}

fn validate_exercise(
    candidate: &ModelExercise,
    request: &ExerciseRequest,
    validators: &[jsonschema::Validator],
) -> Result<Uuid, ExerciseGenerationError> {
    let index = request
        .available_exercises
        .iter()
        .position(|definition| definition.kind == candidate.kind)
        .ok_or_else(|| ExerciseGenerationError::UnknownKind(candidate.kind.clone()))?;
    let errors: Vec<String> = validators[index]
        .iter_errors(&candidate.data)
        .take(8)
        .map(|error| error.to_string())
        .collect();
    if !errors.is_empty() {
        return Err(ExerciseGenerationError::InvalidExercise(errors.join("; ")));
    }
    validation::validate_payload(&candidate.kind, &candidate.data)?;
    if candidate.kind == "transliterate" {
        let script_id = candidate.data["script_id"].as_str().ok_or_else(|| {
            ExerciseGenerationError::InvalidExercise("missing source script".into())
        })?;
        if !request
            .available_scripts
            .iter()
            .any(|script| script.id == script_id)
        {
            return Err(ExerciseGenerationError::InvalidExercise(
                "transliteration source script must be available for the target".into(),
            ));
        }
    }
    Ok(request.available_exercises[index].id)
}

fn resolve_concepts(
    request: &ExerciseRequest,
    slots: &[Uuid],
    data: &Value,
    proposed_concepts: Vec<ModelConcept>,
) -> Result<Vec<Concept>, ExerciseGenerationError> {
    let referenced = validation::concept_ids(data)?;
    let mut concepts = Vec::new();
    let mut codes = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for proposed in proposed_concepts {
        if !slots.contains(&proposed.id)
            || !referenced.contains(&proposed.id)
            || !ids.insert(proposed.id)
            || !codes.insert(proposed.code.clone())
            || request
                .known_concepts
                .iter()
                .any(|concept| concept.code == proposed.code)
            || !request
                .available_skills
                .iter()
                .any(|skill| skill.id == proposed.skill_id)
            || [&proposed.code, &proposed.name, &proposed.description]
                .iter()
                .any(|text| text.trim().is_empty())
        {
            return Err(ExerciseGenerationError::InvalidExercise("new concepts must use distinct supplied slots and codes, known skills, nonblank metadata, and appear in the payload".into()));
        }
        concepts.push(Concept {
            id: proposed.id,
            language_id: request.target_language.clone(),
            skill_id: proposed.skill_id,
            code: proposed.code,
            name: proposed.name,
            description: proposed.description,
        });
    }
    for id in referenced {
        if ids.contains(&id) {
            continue;
        }
        let concept = request
            .known_concepts
            .iter()
            .find(|concept| concept.id == id)
            .ok_or_else(|| {
                ExerciseGenerationError::InvalidExercise(
                    "payload references an unknown concept or an undescribed concept slot".into(),
                )
            })?;
        concepts.push(concept.clone());
    }
    Ok(concepts)
}
