use std::collections::BTreeMap;

use serde::Serialize;
use uuid::Uuid;

use super::SimulationError;
use crate::{exercise_contract, types::Exercise};

#[derive(Debug, Clone)]
pub struct MistakeDistribution {
    default_percentage: u8,
    percentages: BTreeMap<String, u8>,
}

impl MistakeDistribution {
    pub fn new(default_percentage: u8) -> Result<Self, SimulationError> {
        validate_percentage(default_percentage)?;
        Ok(Self {
            default_percentage,
            percentages: BTreeMap::new(),
        })
    }

    pub fn with_rate(mut self, kind: &str, percentage: u8) -> Result<Self, SimulationError> {
        validate_percentage(percentage)?;
        if kind.trim().is_empty() {
            return Err(SimulationError::Configuration(
                "exercise kind cannot be blank".into(),
            ));
        }
        self.percentages.insert(kind.into(), percentage);
        Ok(self)
    }

    pub(super) fn plan(
        &self,
        exercise: &Exercise,
        graded: bool,
    ) -> Result<SimulationPlan, SimulationError> {
        let percentage = if graded {
            self.percentages
                .get(&exercise.kind)
                .copied()
                .unwrap_or(self.default_percentage)
        } else {
            0
        };
        let mut plan = SimulationPlan {
            mistake_percentage: percentage,
            correct_answer: None,
            matching_results: BTreeMap::new(),
        };
        if !graded {
            return Ok(plan);
        }
        if exercise.kind != "match_words" {
            plan.correct_answer = Some(draw(100)? >= u32::from(percentage));
            return Ok(plan);
        }

        let ids: Vec<_> = exercise_contract::concept_ids(&exercise.payload)?
            .into_iter()
            .collect();
        let expected = ids.len() as u32 * u32::from(percentage);
        let mut mistakes = expected / 100 + u32::from(draw(100)? < expected % 100);
        // A complete one-to-one matching cannot contain exactly one wrong pair.
        if mistakes == 1 {
            mistakes = if draw(2)? == 0 { 0 } else { 2 };
        }
        let mut indices: Vec<_> = (0..ids.len()).collect();
        for index in 0..indices.len() {
            let other = index + draw((indices.len() - index) as u32)? as usize;
            indices.swap(index, other);
        }
        for id in &ids {
            plan.matching_results.insert(*id, true);
        }
        for index in indices.into_iter().take(mistakes as usize) {
            plan.matching_results.insert(ids[index], false);
        }
        Ok(plan)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulationPlan {
    pub mistake_percentage: u8,
    pub correct_answer: Option<bool>,
    pub matching_results: BTreeMap<Uuid, bool>,
}

impl SimulationPlan {
    pub fn validate_results(&self, results: &BTreeMap<Uuid, bool>) -> Result<(), SimulationError> {
        let matches = if !self.matching_results.is_empty() {
            &self.matching_results == results
        } else if let Some(expected) = self.correct_answer {
            !results.is_empty() && results.values().all(|correct| *correct) == expected
        } else {
            results.is_empty()
        };
        if !matches {
            return Err(exercise_contract::invalid(
                "simulated answer does not realize the sampled mistake plan",
            )
            .into());
        }
        Ok(())
    }
}

fn validate_percentage(percentage: u8) -> Result<(), SimulationError> {
    if percentage > 100 {
        return Err(SimulationError::Configuration(
            "mistake percentage must be between 0 and 100".into(),
        ));
    }
    Ok(())
}

fn draw(upper: u32) -> Result<u32, SimulationError> {
    let mut bytes = [0; 4];
    getrandom::fill(&mut bytes).map_err(|error| SimulationError::Random(error.to_string()))?;
    Ok(((u64::from(u32::from_le_bytes(bytes)) * u64::from(upper)) >> 32) as u32)
}
