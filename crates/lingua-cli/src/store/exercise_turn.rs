use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::ExerciseTurn;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExerciseTurnStore(Pool<ConnectionManager>);

impl ExerciseTurnStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn set_evaluation(
        &self,
        turn_id: Uuid,
        evaluation: &serde_json::Value,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let evaluation = serde_json::to_string(evaluation)?;
        super::query::changed_one(connection.execute(
            queries::SET_EVALUATION,
            Parameters::positional(&[&evaluation, &turn_id]),
        )?)
    }

    pub fn insert(&self, model: &ExerciseTurn) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let role = model.role.as_str();
        let evaluation = model
            .evaluation
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let created_on = model.created_on.to_rfc3339();
        let changed = connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.id,
                &model.exercise_id,
                &model.turn_number,
                &role,
                &model.content,
                &evaluation,
                &created_on,
            ]),
        )?;
        if changed != 1 {
            return Err(StoreError::InvalidInput(
                "turns require an existing unfinished exercise",
            ));
        }

        Ok(())
    }

    pub fn get(&self, id: Uuid) -> Result<Option<ExerciseTurn>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(&self, exercise_id: Uuid) -> Result<Vec<ExerciseTurn>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&exercise_id]),
        )
    }
}

mod queries {
    pub(super) const SET_EVALUATION: &str = r#"
        UPDATE exercise_turn
        SET
            evaluation = $1
        WHERE
            id = $2
            AND exercise_id IN (SELECT id FROM exercise WHERE answered_on IS NULL)
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO exercise_turn
        (
            id,
            exercise_id,
            turn_number,
            role,
            content,
            evaluation,
            created_on
        )
        SELECT
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7
        FROM exercise
        WHERE
            id = $2
            AND answered_on IS NULL
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            id,
            exercise_id,
            turn_number,
            role,
            content,
            evaluation,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on
        FROM exercise_turn
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            exercise_id,
            turn_number,
            role,
            content,
            evaluation,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on
        FROM exercise_turn
        WHERE
            exercise_id = $1
        ORDER BY
            turn_number
    "#;
}
