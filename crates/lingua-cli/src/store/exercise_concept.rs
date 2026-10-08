use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::ExerciseConcept;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExerciseConceptStore(Pool<ConnectionManager>);

impl ExerciseConceptStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(&self, model: &ExerciseConcept) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let changed = connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.exercise_id, &model.concept_id, &model.is_primary]),
        )?;

        if changed != 1 {
            return Err(StoreError::InvalidInput(
                "exercise and concept must belong to the same target",
            ));
        }

        Ok(())
    }

    pub fn get(
        &self,
        exercise_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<ExerciseConcept>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&exercise_id, &concept_id]),
        )
    }

    pub fn list(&self, exercise_id: Uuid) -> Result<Vec<ExerciseConcept>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&exercise_id]),
        )
    }
}

mod queries {
    pub(super) const INSERT: &str = r#"
        INSERT INTO exercise_concept
        (
            exercise_id,
            concept_id,
            is_primary
        )
        SELECT
            x.id,
            c.id,
            $3
        FROM exercise x
        JOIN session s
            ON s.id = x.session_id
        JOIN concept c
            ON c.language_id = s.target_language_id
        WHERE
            x.id = $1
            AND c.id = $2
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            ec.exercise_id,
            ec.concept_id,
            ec.is_primary
        FROM exercise_concept ec
        WHERE
            ec.exercise_id = $1
            AND ec.concept_id = $2
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            ec.exercise_id,
            ec.concept_id,
            ec.is_primary
        FROM exercise_concept ec
        WHERE
            ec.exercise_id = $1
        ORDER BY
            ec.concept_id
    "#;
}
