use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use uuid::Uuid;

use super::EvaluationError;
use crate::{exercise_contract as contract, types::Exercise};

pub(super) fn validate(
    verdict: &Value,
    exercise: &Exercise,
    answer: &Value,
    expected: &BTreeSet<Uuid>,
) -> Result<BTreeMap<Uuid, bool>, EvaluationError> {
    let items = verdict["concept_results"]
        .as_array()
        .ok_or_else(|| contract::invalid("missing concept results"))?;
    let mut results = BTreeMap::new();
    for item in items {
        let id = contract::parse_id(&item["concept_id"])?;
        let correct = item["correct"]
            .as_bool()
            .ok_or_else(|| contract::invalid("correct must be a boolean"))?;
        if results.insert(id, correct).is_some() {
            return Err(contract::invalid("verdict contains duplicate concept results").into());
        }
    }
    if results.keys().copied().collect::<BTreeSet<_>>() != *expected {
        return Err(contract::invalid("verdict must assess exactly the exercise concepts").into());
    }
    if verdict["feedback"]
        .as_str()
        .is_none_or(|text| text.trim().is_empty())
    {
        return Err(contract::invalid("verdict requires nonblank feedback").into());
    }
    if let Some(objective) = contract::objective_results(exercise, answer)? {
        if objective != results {
            return Err(
                contract::invalid("model verdict contradicts the objective answer key").into(),
            );
        }
    }
    Ok(results)
}
