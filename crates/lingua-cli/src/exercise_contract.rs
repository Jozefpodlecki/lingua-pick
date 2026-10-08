use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

use crate::types::{Exercise, ExerciseDefinition, InteractionMode};

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("{0}")]
    Invalid(String),
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn invalid(message: &str) -> ContractError {
    ContractError::Invalid(message.into())
}

pub fn compile_schema(schema: &Value) -> Result<jsonschema::Validator, ContractError> {
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(schema)
        .map_err(|error| ContractError::Invalid(format!("invalid schema: {error}")))
}

pub fn validate_schema(
    validator: &jsonschema::Validator,
    value: &Value,
) -> Result<(), ContractError> {
    let errors: Vec<_> = validator
        .iter_errors(value)
        .take(8)
        .map(|e| e.to_string())
        .collect();
    if !errors.is_empty() {
        return Err(ContractError::Invalid(errors.join("; ")));
    }
    Ok(())
}

pub fn parse_response(
    response: &str,
    validator: &jsonschema::Validator,
) -> Result<Value, ContractError> {
    if response.len() > 128 * 1024 {
        return Err(invalid("response exceeds 128 KiB"));
    }
    let value = serde_json::from_str(response)?;
    validate_schema(validator, &value)?;
    Ok(value)
}

pub fn validate_exercise(
    definition: &ExerciseDefinition,
    exercise: &Exercise,
) -> Result<BTreeSet<Uuid>, ContractError> {
    if definition.id != exercise.definition_id || definition.kind != exercise.kind {
        return Err(invalid("exercise does not match its definition"));
    }
    if exercise.answered_on.is_some() || exercise.interaction_mode != InteractionMode::SingleTurn {
        return Err(invalid("only pending single-turn exercises are supported"));
    }
    validate_schema(&compile_schema(&definition.schema)?, &exercise.payload)?;
    validate_payload(&exercise.kind, &exercise.payload)?;
    let ids = concept_ids(&exercise.payload)?;
    if ids.is_empty() {
        return Err(invalid("exercise must reference learning concepts"));
    }
    Ok(ids)
}

pub fn concept_ids(value: &Value) -> Result<BTreeSet<Uuid>, ContractError> {
    let mut ids = BTreeSet::new();
    collect_concepts(value, &mut ids)?;
    Ok(ids)
}

fn collect_concepts(value: &Value, ids: &mut BTreeSet<Uuid>) -> Result<(), ContractError> {
    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                match key.as_str() {
                    "concept_id" => {
                        ids.insert(parse_id(value)?);
                    }
                    "concept_ids" => {
                        for id in value
                            .as_array()
                            .ok_or_else(|| invalid("concept_ids must be an array"))?
                        {
                            ids.insert(parse_id(id)?);
                        }
                    }
                    _ => collect_concepts(value, ids)?,
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_concepts(item, ids)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn parse_id(value: &Value) -> Result<Uuid, ContractError> {
    Uuid::parse_str(
        value
            .as_str()
            .ok_or_else(|| invalid("concept reference must be a UUID string"))?,
    )
    .map_err(|_| invalid("concept reference is not a valid UUID"))
}

pub fn validate_answer(exercise: &Exercise, answer: &Value) -> Result<(), ContractError> {
    if let Some(matches) = answer.get("matches").and_then(Value::as_array) {
        let pairs = exercise.payload["pairs"]
            .as_array()
            .ok_or_else(|| invalid("matching pairs are missing"))?;
        let ids = concept_ids(&exercise.payload)?;
        let labels: BTreeSet<_> = pairs.iter().filter_map(|p| p["english"].as_str()).collect();
        let mut submitted_ids = BTreeSet::new();
        let mut submitted_labels = BTreeSet::new();
        for item in matches {
            if !submitted_ids.insert(parse_id(&item["concept_id"])?)
                || !submitted_labels.insert(
                    item["english"]
                        .as_str()
                        .ok_or_else(|| invalid("missing matching label"))?,
                )
            {
                return Err(invalid("matching must be one-to-one"));
            }
        }
        if submitted_ids != ids || submitted_labels != labels {
            return Err(invalid(
                "matching must cover every supplied concept and English label exactly once",
            ));
        }
    }
    if let Some(id) = answer.get("choice_id").and_then(Value::as_str) {
        if !exercise.payload["choices"]
            .as_array()
            .is_some_and(|choices| choices.iter().any(|c| c["id"].as_str() == Some(id)))
        {
            return Err(invalid("answer references an unavailable choice"));
        }
    }
    if let Some(sequence) = answer.get("token_ids").and_then(Value::as_array) {
        let tokens = exercise.payload["tokens"]
            .as_array()
            .ok_or_else(|| invalid("missing tokens"))?;
        let available: BTreeSet<_> = tokens.iter().filter_map(|t| t["id"].as_str()).collect();
        let mut used = BTreeSet::new();
        for id in sequence {
            let id = id
                .as_str()
                .ok_or_else(|| invalid("token ID must be a string"))?;
            if !available.contains(id) || !used.insert(id) {
                return Err(invalid("answer contains an unknown or repeated token"));
            }
        }
        if exercise.kind == "order_words" && used != available {
            return Err(invalid("word order must use every token exactly once"));
        }
    }
    if answer
        .get("text")
        .and_then(Value::as_str)
        .is_some_and(|s| s.trim().is_empty())
    {
        return Err(invalid("answer text cannot be blank"));
    }
    Ok(())
}

pub fn objective_results(
    exercise: &Exercise,
    answer: &Value,
) -> Result<Option<BTreeMap<Uuid, bool>>, ContractError> {
    let data = &exercise.payload;
    if exercise.kind == "match_words" {
        let pairs = data["pairs"]
            .as_array()
            .ok_or_else(|| invalid("missing pairs"))?;
        let matches = answer["matches"]
            .as_array()
            .ok_or_else(|| invalid("missing matches"))?;
        let mut results = BTreeMap::new();
        for pair in pairs {
            let id = parse_id(&pair["concept_id"])?;
            let submitted = matches
                .iter()
                .find(|m| m["concept_id"] == pair["concept_id"]);
            results.insert(
                id,
                submitted.is_some_and(|m| m["english"] == pair["english"]),
            );
        }
        return Ok(Some(results));
    }
    let correct = if data.get("correct_choice_id").is_some() {
        Some(answer["choice_id"] == data["correct_choice_id"])
    } else if let Some(orders) = data.get("accepted_orders").and_then(Value::as_array) {
        Some(orders.contains(&answer["token_ids"]))
    } else if exercise.kind == "transliterate" {
        let normalize = |text: &str| {
            if data["case_sensitive"].as_bool() == Some(true) {
                text.trim().to_owned()
            } else {
                text.trim().to_lowercase()
            }
        };
        let text = normalize(
            answer["text"]
                .as_str()
                .ok_or_else(|| invalid("missing transliteration answer"))?,
        );
        Some(data["accepted_answers"].as_array().is_some_and(|values| {
            values
                .iter()
                .any(|v| v.as_str().is_some_and(|s| normalize(s) == text))
        }))
    } else {
        None
    };
    Ok(correct
        .map(|correct| {
            concept_ids(data).map(|ids| ids.into_iter().map(|id| (id, correct)).collect())
        })
        .transpose()?)
}
fn validate_strings(value: &Value) -> Result<(), ContractError> {
    match value {
        Value::String(text) if text.trim().is_empty() || text.chars().count() > 4096 => {
            return Err(invalid(
                "payload strings must be nonblank and at most 4096 characters",
            ));
        }
        Value::Array(items) => {
            for item in items {
                validate_strings(item)?;
            }
        }
        Value::Object(fields) => {
            for value in fields.values() {
                validate_strings(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn unique_strings(items: &[Value], field: &str) -> Result<BTreeSet<String>, ContractError> {
    let mut found = BTreeSet::new();
    for item in items {
        let text = item
            .get(field)
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("missing item identifier or label"))?;
        if !found.insert(text.trim().to_lowercase()) {
            return Err(invalid("item identifiers and labels must be distinct"));
        }
    }
    Ok(found)
}

pub fn validate_payload(kind: &str, data: &Value) -> Result<(), ContractError> {
    validate_strings(data)?;
    if concept_ids(data)?.is_empty() {
        return Err(invalid(
            "exercise must identify at least one learning concept",
        ));
    }
    if let Some(pairs) = data.get("pairs").and_then(Value::as_array) {
        unique_strings(pairs, "target")?;
        unique_strings(pairs, "english")?;
        unique_strings(pairs, "concept_id")?;
    }
    if let Some(choices) = data.get("choices").and_then(Value::as_array) {
        unique_strings(choices, "id")?;
        unique_strings(choices, "text")?;
        let correct = data
            .get("correct_choice_id")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("missing correct choice"))?;
        if !choices
            .iter()
            .any(|choice| choice.get("id").and_then(Value::as_str) == Some(correct))
        {
            return Err(invalid(
                "correct_choice_id must reference an existing choice",
            ));
        }
    }
    if let Some(tokens) = data.get("tokens").and_then(Value::as_array) {
        unique_strings(tokens, "id")?;
        let token_ids: BTreeSet<&str> = tokens
            .iter()
            .filter_map(|token| token.get("id").and_then(Value::as_str))
            .collect();
        let orders = data
            .get("accepted_orders")
            .and_then(Value::as_array)
            .ok_or_else(|| invalid("token exercise requires accepted orders"))?;
        for order in orders {
            let sequence = order
                .as_array()
                .ok_or_else(|| invalid("accepted token order must be an array"))?;
            let mut used = BTreeSet::new();
            for id in sequence {
                let id = id
                    .as_str()
                    .ok_or_else(|| invalid("token IDs must be strings"))?;
                if !token_ids.contains(id) || !used.insert(id) {
                    return Err(invalid(
                        "accepted orders must use existing token IDs at most once",
                    ));
                }
            }
            if kind == "order_words" && used != token_ids {
                return Err(invalid(
                    "word-order answers must use every token exactly once",
                ));
            }
        }
    }
    Ok(())
}
