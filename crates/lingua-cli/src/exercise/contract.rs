use std::collections::BTreeSet;
use uuid::Uuid;

use super::ContractError;
use crate::{exercise::*, types::{Exercise, ExerciseDefinition, InteractionMode}};

type Result<T> = std::result::Result<T, ContractError>;

pub fn validate_exercise(
    definition: &ExerciseDefinition,
    exercise: &Exercise,
) -> Result<BTreeSet<Uuid>> {
    if definition.id != exercise.definition_id || definition.kind != exercise.kind {
        return Err(ContractError::DefinitionMismatch);
    }

    if exercise.answered_on.is_some()
        || exercise.interaction_mode != InteractionMode::SingleTurn
    {
        return Err(ContractError::UnsupportedExerciseState);
    }

    validate_schema(&compile_schema(&definition.schema)?, &exercise.payload)?;
    validate_payload(&exercise.kind, &exercise.payload)?;

    let ids = concept_ids(&exercise.payload)?;

    if ids.is_empty() {
        return Err(ContractError::MissingConcepts);
    }

    Ok(ids)
}