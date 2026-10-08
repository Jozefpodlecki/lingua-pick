use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::{Exercise, LearningEvidence};
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExerciseStore(Pool<ConnectionManager>);

impl ExerciseStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_last_for_session(&self, session_id: Uuid) -> Result<Option<Exercise>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_LAST_FOR_SESSION,
            Parameters::positional(&[&session_id]),
        )
    }

    pub fn set_answer(
        &self,
        exercise_id: Uuid,
        answer: &serde_json::Value,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let answer = serde_json::to_string(answer)?;
        super::query::changed_one(connection.execute(
            queries::SET_ANSWER,
            Parameters::positional(&[&answer, &exercise_id]),
        )?)
    }

    pub fn complete(
        &self,
        exercise_id: Uuid,
        answer: &serde_json::Value,
        verdict: &serde_json::Value,
    ) -> Result<(), StoreError> {
        self.complete_with_evidence(exercise_id, Some(answer), verdict, &[])
    }

    pub fn complete_ungraded(
        &self,
        exercise_id: Uuid,
        verdict: &serde_json::Value,
    ) -> Result<(), StoreError> {
        self.complete_with_evidence(exercise_id, None, verdict, &[])
    }

    pub fn complete_with_evidence(
        &self,
        exercise_id: Uuid,
        answer: Option<&serde_json::Value>,
        verdict: &serde_json::Value,
        evidence: &[LearningEvidence],
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let answer_json = answer.map(serde_json::to_string).transpose()?;
        let verdict_json = serde_json::to_string(verdict)?;

        super::query::transaction(&connection, || {
            let exercise: Exercise = read_one(
                &connection,
                queries::GET_FOR_COMPLETION,
                Parameters::positional(&[&exercise_id]),
            )?
            .ok_or(StoreError::NotFound)?;
            if exercise.answered_on.is_some() {
                if exercise.answer.as_ref() == answer
                    && exercise.verdict.as_ref() == Some(verdict)
                    && evidence.is_empty()
                {
                    return Ok(());
                }
                return Err(StoreError::InvalidInput(
                    "completed exercise outcomes are immutable",
                ));
            }

            let graded = super::query::has_rows(
                &connection,
                queries::IS_GRADED,
                Parameters::positional(&[&exercise.definition_id]),
            )?;
            if graded != answer.is_some() {
                return Err(StoreError::InvalidInput(
                    "graded exercises require an answer; ungraded exercises must not have one",
                ));
            }
            for record in evidence {
                if record.exercise_id != exercise_id || (!graded && record.correct.is_some()) {
                    return Err(StoreError::InvalidInput(
                        "evidence does not match the completed exercise",
                    ));
                }
                super::learning_evidence::insert_evidence(&connection, record)?;
                super::user_concept::refresh_from_evidence(
                    &connection,
                    record.user_id,
                    &record.target_language_id,
                )?;
            }
            super::query::changed_one(connection.execute(
                queries::SAVE_OUTCOME,
                Parameters::positional(&[&answer_json, &verdict_json, &exercise_id]),
            )?)?;
            super::session::refresh_progress(&connection, exercise.session_id)
        })
    }

    pub fn insert(&self, model: &Exercise) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let created_on = model.created_on.to_rfc3339();
        let answered_on = model.answered_on.map(|value| value.to_rfc3339());
        let payload = serde_json::to_string(&model.payload)?;
        let answer = model
            .answer
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let verdict = model
            .verdict
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let interaction_mode = model.interaction_mode.as_str();
        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.id,
                &model.definition_id,
                &model.session_id,
                &created_on,
                &answered_on,
                &model.kind,
                &payload,
                &answer,
                &verdict,
                &interaction_mode,
            ]),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<Exercise>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_ID,
            Parameters::positional(&[&id]),
        )
    }

    pub fn list(&self, session_id: Uuid) -> Result<Vec<Exercise>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&session_id]),
        )
    }
}

mod queries {
    pub(super) const GET_LAST_FOR_SESSION: &str = r#"
        SELECT
            id,
            definition_id,
            session_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(answered_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS answered_on,
            kind,
            payload,
            answer,
            verdict,
            interaction_mode
        FROM exercise
        WHERE
            session_id = $1
        ORDER BY
            created_on DESC,
            id DESC
        LIMIT 1
    "#;

    pub(super) const SET_ANSWER: &str = r#"
        UPDATE exercise
        SET
            answer = $1
        WHERE
            id = $2
            AND answered_on IS NULL
            AND definition_id IN (SELECT id FROM exercise_definition WHERE answer_schema IS NOT NULL)
    "#;

    pub(super) const GET_FOR_COMPLETION: &str = r#"
        SELECT
            id,
            definition_id,
            session_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(answered_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS answered_on,
            kind,
            payload,
            answer,
            verdict,
            interaction_mode
        FROM exercise
        WHERE
            id = $1
    "#;

    pub(super) const IS_GRADED: &str = r#"
        SELECT
            1
        FROM exercise_definition
        WHERE
            id = $1
            AND answer_schema IS NOT NULL
    "#;

    pub(super) const SAVE_OUTCOME: &str = r#"
        UPDATE exercise
        SET
            answer = $1,
            verdict = $2,
            answered_on = current_timestamp
        WHERE
            id = $3
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO exercise
        (
            id,
            definition_id,
            session_id,
            created_on,
            answered_on,
            kind,
            payload,
            answer,
            verdict,
            interaction_mode
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10
        )
    "#;

    pub(super) const GET_BY_ID: &str = r#"
        SELECT
            id,
            definition_id,
            session_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(answered_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS answered_on,
            kind,
            payload,
            answer,
            verdict,
            interaction_mode
        FROM exercise
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            definition_id,
            session_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(answered_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS answered_on,
            kind,
            payload,
            answer,
            verdict,
            interaction_mode
        FROM exercise
        WHERE
            session_id = $1
        ORDER BY
            created_on,
            id
    "#;
}
