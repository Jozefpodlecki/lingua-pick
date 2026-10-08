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
    new_concepts: Vec<ModelConcept>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelConcept {
    id: Uuid,
    code: String,
    name: String,
    description: String,
    skill_id: String,
    topic_id: String,
}

pub(super) fn decode(
    response: String,
    request: &ExerciseRequest,
    validators: &[jsonschema::Validator],
    slots: &[Uuid],
) -> Result<GenerationResult, ExerciseGenerationError> {
    let candidate = parse_response(response)?;
    let definition_id = validate_exercise(&candidate, request, validators)?;
    let concepts = resolve_concepts(request, slots, &candidate.data, candidate.new_concepts)?;

    Ok(GenerationResult {
        exercise: GeneratedExercise {
            definition_id,
            kind: candidate.kind,
            payload: candidate.data,
        },
        concepts,
    })
}

fn invalid(message: impl Into<String>) -> ExerciseGenerationError {
    ExerciseGenerationError::InvalidExercise(message.into())
}

fn parse_response(response: String) -> Result<ModelExercise, ExerciseGenerationError> {
    if response.len() > MAX_RESPONSE_BYTES {
        return Err(invalid("response exceeds 128 KiB"));
    }

    serde_json::from_str(&response).map_err(|error| ExerciseGenerationError::InvalidResponse {
        message: error.to_string(),
        response,
    })
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

    let validator = validators
        .get(index)
        .ok_or_else(|| invalid("missing validator for exercise definition"))?;

    let errors: Vec<String> = validator
        .iter_errors(&candidate.data)
        .take(8)
        .map(|error| error.to_string())
        .collect();

    if !errors.is_empty() {
        return Err(invalid(errors.join("; ")));
    }

    validation::validate_payload(&candidate.kind, &candidate.data)?;
    validate_script(candidate, request)?;

    Ok(request.available_exercises[index].id)
}

fn validate_script(
    candidate: &ModelExercise,
    request: &ExerciseRequest,
) -> Result<(), ExerciseGenerationError> {
    if candidate.kind != "transliterate" {
        return Ok(());
    }

    let script_id = candidate.data["script_id"]
        .as_str()
        .ok_or_else(|| invalid("missing source script"))?;

    if !request
        .available_scripts
        .iter()
        .any(|script| script.id == script_id)
    {
        return Err(invalid(
            "transliteration source script must be available for the target",
        ));
    }

    Ok(())
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
    let mut errors = Vec::new();

    for (index, proposed) in proposed_concepts.into_iter().enumerate() {
        validate_new_concept(
            &proposed,
            index,
            request,
            slots,
            &referenced,
            &mut ids,
            &mut codes,
            &mut errors,
        );

        concepts.push(Concept {
            id: proposed.id,
            language_id: request.target_language.clone(),
            skill_id: proposed.skill_id,
            topic_id: proposed.topic_id,
            code: proposed.code,
            name: proposed.name,
            description: proposed.description,
        });
    }

    resolve_known_concepts(request, data, &referenced, &ids, &mut concepts, &mut errors);

    if !errors.is_empty() {
        return Err(invalid(format!(
            "concept resolution failed: {}",
            errors.join("; ")
        )));
    }

    Ok(concepts)
}

fn validate_new_concept(
    proposed: &ModelConcept,
    index: usize,
    request: &ExerciseRequest,
    slots: &[Uuid],
    referenced: &BTreeSet<Uuid>,
    ids: &mut BTreeSet<Uuid>,
    codes: &mut BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let existing = request
        .known_concepts
        .iter()
        .find(|concept| concept.code == proposed.code);
    let mut report = |field: &str, rule: &str, message: &str| {
        if errors.len() >= 16 {
            return;
        }

        let path = format!("$.new_concepts[{index}].{field}");
        tracing::warn!(
            stage = "concept_resolution",
            %path,
            rule,
            concept_id = %proposed.id,
            code = %proposed.code,
            code_already_retrieved = existing.is_some(),
            existing_concept_id = ?existing.map(|concept| concept.id),
            detail = message,
            "Invalid new concept"
        );
        errors.push(format!(
            "{path} (rule: {rule}, id: {}, code: {}): {message}",
            proposed.id, proposed.code
        ));
    };

    if !slots.contains(&proposed.id) {
        report(
            "id",
            "allocated_new_concept_id",
            "new concept uses an unallocated ID",
        );
    }

    if !referenced.contains(&proposed.id) {
        report(
            "id",
            "new_concept_must_be_referenced",
            "new concept is not referenced by the exercise",
        );
    }

    if !ids.insert(proposed.id) {
        report("id", "unique_new_concept_id", "duplicate new concept ID");
    }

    if !codes.insert(proposed.code.clone()) {
        report(
            "code",
            "unique_new_concept_code",
            "duplicate new concept code",
        );
    }

    if existing.is_some() {
        report(
            "code",
            "reuse_existing_concept",
            "code already belongs to a retrieved concept; reuse its ID and omit new metadata",
        );
    }

    if !request
        .available_skills
        .iter()
        .any(|skill| skill.id == proposed.skill_id)
    {
        report(
            "skill_id",
            "available_concept_skill",
            "unknown concept skill",
        );
    }

    if !request
        .available_topics
        .iter()
        .any(|topic| topic.id == proposed.topic_id)
    {
        report(
            "topic_id",
            "available_concept_topic",
            "unknown concept topic",
        );
    }

    for (field, text) in [
        ("code", &proposed.code),
        ("name", &proposed.name),
        ("description", &proposed.description),
    ] {
        if text.trim().is_empty() {
            report(
                field,
                "nonblank_concept_metadata",
                "concept metadata must not be blank",
            );
        }
    }
}

fn resolve_known_concepts(
    request: &ExerciseRequest,
    data: &Value,
    referenced: &BTreeSet<Uuid>,
    new_ids: &BTreeSet<Uuid>,
    concepts: &mut Vec<Concept>,
    errors: &mut Vec<String>,
) {
    for id in referenced.difference(new_ids) {
        let concept = request
            .known_concepts
            .iter()
            .find(|concept| concept.id == *id);

        if let Some(concept) = concept {
            concepts.push(concept.clone());
        } else if errors.len() < 16 {
            let mut paths = Vec::new();
            reference_paths(data, "$.data", *id, &mut paths);
            for path in paths.into_iter().take(16 - errors.len()) {
                tracing::warn!(
                    stage = "concept_resolution",
                    %path,
                    rule = "known_or_described_concept",
                    concept_id = %id,
                    "Payload references an unknown concept or an undescribed concept slot"
                );
                errors.push(format!("{path} (rule: known_or_described_concept, id: {id}): unknown concept or undescribed concept slot"));
            }
        }
    }
}

fn reference_paths(value: &Value, path: &str, id: Uuid, paths: &mut Vec<String>) {
    if paths.len() >= 16 {
        return;
    }

    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                let child = format!("{path}.{key}");
                if key == "concept_id" {
                    if value.as_str().and_then(|text| Uuid::parse_str(text).ok()) == Some(id) {
                        paths.push(child);
                    }
                } else if key == "concept_ids" {
                    if let Some(ids) = value.as_array() {
                        for (index, value) in ids.iter().enumerate() {
                            if paths.len() < 16
                                && value.as_str().and_then(|text| Uuid::parse_str(text).ok())
                                    == Some(id)
                            {
                                paths.push(format!("{child}[{index}]"));
                            }
                        }
                    }
                } else {
                    reference_paths(value, &child, id, paths);
                }
            }
        }
        Value::Array(items) => {
            for (index, value) in items.iter().enumerate() {
                reference_paths(value, &format!("{path}[{index}]"), id, paths);
            }
        }
        _ => {}
    }
}
