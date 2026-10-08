use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use uuid::Uuid;

use super::EvaluationError;
use crate::{exercise, types::Exercise};

pub(super) fn validate(
    verdict: &Value,
    exercise: &Exercise,
    answer: &Value,
    expected: &BTreeSet<Uuid>,
) -> Result<BTreeMap<Uuid, bool>, EvaluationError> {
    let items = verdict["concept_results"]
        .as_array()
        .ok_or(EvaluationError::MissingConceptResults)?;

    let mut results = BTreeMap::new();

    for item in items {
        let id = exercise::parse_id(&item["concept_id"])?;

        let correct = item["correct"]
            .as_bool()
            .ok_or(EvaluationError::InvalidConceptResult)?;

        if results.insert(id, correct).is_some() {
            return Err(EvaluationError::DuplicateConceptResult { id });
        }
    }

    let actual: BTreeSet<_> = results.keys().copied().collect();

    if actual != *expected {
        return Err(EvaluationError::ConceptResultsMismatch {
            missing: expected.difference(&actual).copied().collect(),
            unexpected: actual.difference(expected).copied().collect(),
        });
    }

    if verdict["feedback"]
        .as_str()
        .is_none_or(|text| text.trim().is_empty())
    {
        return Err(EvaluationError::MissingFeedback);
    }

    if let Some(objective) = exercise::objective_results(exercise, answer)? {
        for (id, expected_correct) in objective {
            let actual_correct = results[&id];

            if actual_correct != expected_correct {
                return Err(EvaluationError::ObjectiveResultMismatch {
                    concept_id: id,
                    expected: expected_correct,
                    actual: actual_correct,
                });
            }
        }
    }

    Ok(results)
}