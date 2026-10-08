use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{
    Result,
    concepts::concept_ids,
    fields::{required_array, required_string},
};
use crate::exercise::ContractError;

pub fn validate_payload(kind: &str, data: &Value) -> Result<()> {
    validate_strings(data)?;

    if concept_ids(data)?.is_empty() {
        return Err(ContractError::MissingConcepts);
    }

    if let Some(pairs) = data.get("pairs") {
        let pairs = pairs
            .as_array()
            .ok_or_else(|| ContractError::InvalidFieldType {
                field: "pairs".into(),
                expected: "an array",
            })?;

        unique_strings(pairs, "pairs", "pair_id")?;
        unique_strings(pairs, "pairs", "target")?;
        unique_strings(pairs, "pairs", "english")?;
        unique_strings(pairs, "pairs", "concept_id")?;
    }

    if let Some(choices) = data.get("choices") {
        let choices = choices
            .as_array()
            .ok_or_else(|| ContractError::InvalidFieldType {
                field: "choices".into(),
                expected: "an array",
            })?;

        unique_strings(choices, "choices", "id")?;
        unique_strings(choices, "choices", "text")?;

        let correct = required_string(data, "correct_choice_id")?;

        if !choices
            .iter()
            .any(|choice| choice["id"].as_str() == Some(correct))
        {
            return Err(ContractError::UnknownReference {
                field: "correct_choice_id".into(),
                value: correct.into(),
            });
        }
    }

    if let Some(tokens) = data.get("tokens") {
        let tokens = tokens
            .as_array()
            .ok_or_else(|| ContractError::InvalidFieldType {
                field: "tokens".into(),
                expected: "an array",
            })?;

        let available = unique_strings(tokens, "tokens", "id")?;
        let orders = required_array(data, "accepted_orders")?;

        for order in orders {
            let sequence = order
                .as_array()
                .ok_or_else(|| ContractError::InvalidFieldType {
                    field: "accepted_orders".into(),
                    expected: "an array of arrays",
                })?;

            let mut used = BTreeSet::new();

            for value in sequence {
                let id = value
                    .as_str()
                    .ok_or_else(|| ContractError::InvalidFieldType {
                        field: "accepted_orders".into(),
                        expected: "arrays of string IDs",
                    })?;

                if !available.contains(id) || !used.insert(id) {
                    return Err(ContractError::InvalidAcceptedOrder);
                }
            }

            if kind == "order_words" && used != available.iter().map(String::as_str).collect() {
                return Err(ContractError::InvalidAcceptedOrder);
            }
        }
    }

    if kind == "transliterate" {
        let answers = required_array(data, "accepted_answers")?;

        if answers.is_empty() {
            return Err(ContractError::MissingField {
                field: "accepted_answers".into(),
            });
        }

        for answer in answers {
            if answer.as_str().is_none() {
                return Err(ContractError::InvalidFieldType {
                    field: "accepted_answers".into(),
                    expected: "an array of strings",
                });
            }
        }
    }

    Ok(())
}

fn validate_strings(value: &Value) -> Result<()> {
    const MAX_STRING_LENGTH: usize = 4096;

    match value {
        Value::String(text) => {
            if text.trim().is_empty() {
                return Err(ContractError::BlankString {
                    field: "payload".into(),
                });
            }

            if text.chars().count() > MAX_STRING_LENGTH {
                return Err(ContractError::StringTooLong {
                    field: "payload".into(),
                    max: MAX_STRING_LENGTH,
                });
            }
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

fn unique_strings(items: &[Value], collection: &str, field: &str) -> Result<BTreeSet<String>> {
    let mut found = BTreeMap::<String, (usize, String)>::new();

    for (index, item) in items.iter().enumerate() {
        let value = required_string(item, field)?;
        let normalized = value.trim().to_lowercase();

        if let Some((first_index, first_value)) = found.get(&normalized) {
            let path = format!("$.data.{collection}[{index}].{field}");
            let conflicting_path = format!("$.data.{collection}[{first_index}].{field}");
            let rule = format!("unique_{collection}_{field}");

            tracing::warn!(
                stage = "payload_validation",
                %path,
                %conflicting_path,
                %rule,
                field,
                index,
                value,
                normalized,
                first_index,
                first_value,
                item = %item,
                conflicting_item = %items[*first_index],
                "Duplicate payload value"
            );

            return Err(ContractError::DuplicateValue {
                field: field.into(),
                value: value.into(),
                path,
                conflicting_path,
                rule,
            });
        }

        found.insert(normalized, (index, value.to_owned()));
    }

    Ok(found.into_keys().collect())
}
