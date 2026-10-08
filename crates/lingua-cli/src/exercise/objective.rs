use std::collections::BTreeMap;

use serde_json::Value;
use uuid::Uuid;

use super::{
    Result,
    concepts::{concept_ids, parse_id},
    fields::{required, required_array, required_string},
};
use crate::{
    exercise::ContractError,
    types::Exercise,
};

pub fn objective_results(
    exercise: &Exercise,
    answer: &Value,
) -> Result<Option<BTreeMap<Uuid, bool>>> {
    let data = &exercise.payload;

    if exercise.kind == "match_words" {
        let pairs = required_array(data, "pairs")?;
        let matches = required_array(answer, "matches")?;

        let mut results = BTreeMap::new();

        for pair in pairs {
            let id = parse_id(required(pair, "concept_id")?)?;
            let pair_id = required_string(pair, "pair_id")?;
            let expected = required_string(pair, "english")?;

            let submitted = matches.iter().find(|item| {
                item["pair_id"].as_str() == Some(pair_id)
            });

            let correct = submitted
                .and_then(|item| item["english"].as_str())
                == Some(expected);

            if results.insert(id, correct).is_some() {
                return Err(ContractError::ConflictingConceptResult { id });
            }
        }

        return Ok(Some(results));
    }

    let correct = if data.get("correct_choice_id").is_some() {
        let expected = required_string(data, "correct_choice_id")?;
        let submitted = required_string(answer, "choice_id")?;

        Some(submitted == expected)
    } else if data.get("accepted_orders").is_some() {
        let orders = required_array(data, "accepted_orders")?;
        let submitted = required_array(answer, "token_ids")?;

        Some(orders.iter().any(|order| {
            order
                .as_array()
                .is_some_and(|items| items.as_slice() == submitted)
        }))
    } else if exercise.kind == "transliterate" {
        let submitted = required_string(answer, "text")?;
        let accepted = required_array(data, "accepted_answers")?;
        let case_sensitive = data["case_sensitive"].as_bool() == Some(true);

        let normalize = |text: &str| {
            if case_sensitive {
                text.trim().to_owned()
            } else {
                text.trim().to_lowercase()
            }
        };

        let submitted = normalize(submitted);
        let mut correct = false;

        for value in accepted {
            let expected = value.as_str().ok_or_else(|| {
                ContractError::InvalidFieldType {
                    field: "accepted_answers".into(),
                    expected: "an array of strings",
                }
            })?;

            if normalize(expected) == submitted {
                correct = true;
                break;
            }
        }

        Some(correct)
    } else {
        None
    };

    correct
        .map(|correct| {
            Ok(concept_ids(data)?
                .into_iter()
                .map(|id| (id, correct))
                .collect())
        })
        .transpose()
}