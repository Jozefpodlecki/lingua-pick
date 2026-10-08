use std::collections::BTreeSet;

use serde_json::Value;

use super::{
    Result,
    fields::{required_array, required_string},
};
use crate::{
    exercise::ContractError,
    types::Exercise,
};

pub fn validate_answer(
    exercise: &Exercise,
    answer: &Value,
) -> Result<()> {
    match exercise.kind.as_str() {
        "match_words" => validate_matching_answer(exercise, answer),

        "order_words" => validate_token_answer(exercise, answer),

        "transliterate" => {
            let text = required_string(answer, "text")?;

            if text.trim().is_empty() {
                return Err(ContractError::BlankString {
                    field: "text".into(),
                });
            }

            Ok(())
        }

        _ if exercise.payload.get("choices").is_some() => {
            let id = required_string(answer, "choice_id")?;
            let choices = required_array(&exercise.payload, "choices")?;

            if !choices
                .iter()
                .any(|choice| choice["id"].as_str() == Some(id))
            {
                return Err(ContractError::UnknownReference {
                    field: "choice_id".into(),
                    value: id.into(),
                });
            }

            Ok(())
        }

        _ if exercise.payload.get("tokens").is_some() => {
            validate_token_answer(exercise, answer)
        }

        _ => Ok(()),
    }
}

fn validate_matching_answer(
    exercise: &Exercise,
    answer: &Value,
) -> Result<()> {
    let pairs = required_array(&exercise.payload, "pairs")?;
    let matches = required_array(answer, "matches")?;

    let expected_ids: BTreeSet<_> = pairs
        .iter()
        .map(|pair| required_string(pair, "pair_id"))
        .collect::<Result<_>>()?;

    let expected_labels: BTreeSet<_> = pairs
        .iter()
        .map(|pair| required_string(pair, "english"))
        .collect::<Result<_>>()?;

    let mut submitted_ids = BTreeSet::new();
    let mut submitted_labels = BTreeSet::new();

    for item in matches {
        let pair_id = required_string(item, "pair_id")?;
        let label = required_string(item, "english")?;

        if !expected_ids.contains(pair_id) {
            return Err(ContractError::UnknownReference {
                field: "pair_id".into(),
                value: pair_id.into(),
            });
        }

        if !expected_labels.contains(label) {
            return Err(ContractError::UnknownReference {
                field: "english".into(),
                value: label.into(),
            });
        }

        if !submitted_ids.insert(pair_id) || !submitted_labels.insert(label) {
            return Err(ContractError::InvalidMatching);
        }
    }

    if submitted_ids != expected_ids || submitted_labels != expected_labels {
        return Err(ContractError::IncompleteAnswer);
    }

    Ok(())
}

fn validate_token_answer(
    exercise: &Exercise,
    answer: &Value,
) -> Result<()> {
    let tokens = required_array(&exercise.payload, "tokens")?;
    let sequence = required_array(answer, "token_ids")?;

    let available: BTreeSet<_> = tokens
        .iter()
        .map(|token| required_string(token, "id"))
        .collect::<Result<_>>()?;

    let mut used = BTreeSet::new();

    for value in sequence {
        let id = value.as_str().ok_or_else(|| {
            ContractError::InvalidFieldType {
                field: "token_ids".into(),
                expected: "an array of strings",
            }
        })?;

        if !available.contains(id) {
            return Err(ContractError::UnknownReference {
                field: "token_ids".into(),
                value: id.into(),
            });
        }

        if !used.insert(id) {
            return Err(ContractError::DuplicateReference {
                field: "token_ids".into(),
                value: id.into(),
            });
        }
    }

    if exercise.kind == "order_words" && used != available {
        return Err(ContractError::IncompleteAnswer);
    }

    Ok(())
}