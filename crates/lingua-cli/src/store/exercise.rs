use chrono::{DateTime, Utc};
use duckdb_neo::{
    Parameters, r2d2::ConnectionManager, types::TimestampTzValue,
};
use r2d2::Pool;
use serde_json::Value;
use uuid::Uuid;

use crate::{store::StoreError, types::Exercise};

#[derive(Clone)]
pub struct ExerciseStore(Pool<ConnectionManager>);

impl ExerciseStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(&self, model: &Exercise) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let payload = serde_json::to_string(&model.payload)?;
        let answer = model.answer.as_ref().map(serde_json::to_string).transpose()?;
        let verdict = model.verdict.as_ref().map(serde_json::to_string).transpose()?;
        let answered_on = model.answered_on.map(|value| value.to_rfc3339());

        connection.execute(
            r#"
            INSERT INTO exercise (
                id,
                session_id,
                created_on,
                answered_on,
                kind,
                payload,
                answer,
                verdict
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            Parameters::positional(&[
                &model.id,
                &model.session_id,
                &model.created_on.to_rfc3339(),
                &answered_on,
                &model.kind,
                &payload,
                &answer,
                &verdict,
            ]),
        )?;

        Ok(())
    }

    pub fn get_by_id(&self, exercise_id: Uuid) -> Result<Option<Exercise>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                id,
                session_id,
                created_on,
                answered_on,
                kind,
                payload::VARCHAR,
                answer::VARCHAR,
                verdict::VARCHAR
            FROM exercise
            WHERE id = $1
            "#,
            Parameters::positional(&[&exercise_id]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? == 0 {
                continue;
            }

            let id = chunk.get_vector_at::<Uuid>(0)?.get(0)?.unwrap();
            let session_id = chunk.get_vector_at::<Uuid>(1)?.get(0)?.unwrap();

            let created_on = chunk.get_vector_at::<TimestampTzValue>(2)?.get(0)?.unwrap();
            let created_on = Option::<DateTime<Utc>>::from(*created_on).unwrap();

            let answered_on = chunk.get_vector_at::<TimestampTzValue>(3)?.get(0)?;
            let answered_on = answered_on.and_then(|value| Option::<DateTime<Utc>>::from(*value));

            let kind = chunk.get_vector_at::<String>(4)?.get(0)?.unwrap().into();

            let payload = chunk.get_vector_at::<String>(5)?.get(0)?.unwrap();
            let payload = serde_json::from_str(payload)?;

            let answer = chunk.get_vector_at::<String>(6)?.get(0)?.map(serde_json::from_str).transpose()?;
            let verdict = chunk.get_vector_at::<String>(7)?.get(0)?.map(serde_json::from_str).transpose()?;

            return Ok(Some(Exercise {
                id,
                session_id,
                created_on,
                answered_on,
                kind,
                payload,
                answer,
                verdict,
            }));
        }

        Ok(None)
    }

    pub fn get_last_for_session(&self, session_id: Uuid) -> Result<Option<Exercise>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                id,
                session_id,
                created_on,
                answered_on,
                kind,
                payload::VARCHAR,
                answer::VARCHAR,
                verdict::VARCHAR
            FROM exercise
            WHERE session_id = $1
            ORDER BY created_on DESC
            LIMIT 1
            "#,
            Parameters::positional(&[&session_id]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? == 0 {
                continue;
            }

            let id = chunk.get_vector_at::<Uuid>(0)?.get(0)?.unwrap();
            let session_id = chunk.get_vector_at::<Uuid>(1)?.get(0)?.unwrap();

            let created_on = chunk.get_vector_at::<TimestampTzValue>(2)?.get(0)?.unwrap();
            let created_on = Option::<DateTime<Utc>>::from(*created_on).unwrap();

            let answered_on = chunk.get_vector_at::<TimestampTzValue>(3)?.get(0)?;
            let answered_on = answered_on.and_then(|value| Option::<DateTime<Utc>>::from(*value));

            let kind = chunk.get_vector_at::<String>(4)?.get(0)?.unwrap().into();

            let payload = chunk.get_vector_at::<String>(5)?.get(0)?.unwrap();
            let payload = serde_json::from_str(payload)?;

            let answer = chunk.get_vector_at::<String>(6)?.get(0)?.map(serde_json::from_str).transpose()?;
            let verdict = chunk.get_vector_at::<String>(7)?.get(0)?.map(serde_json::from_str).transpose()?;

            return Ok(Some(Exercise {
                id,
                session_id,
                created_on,
                answered_on,
                kind,
                payload,
                answer,
                verdict,
            }));
        }

        Ok(None)
    }

    pub fn update_answer(
        &self,
        exercise_id: Uuid,
        answer: &Value,
        verdict: &Value,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let answer = serde_json::to_string(answer)?;
        let verdict = serde_json::to_string(verdict)?;

        connection.execute(
            r#"
            UPDATE exercise
            SET
                answer = $1,
                verdict = $2,
                answered_on = current_timestamp
            WHERE id = $3
            "#,
            Parameters::positional(&[
                &answer,
                &verdict,
                &exercise_id,
            ]),
        )?;

        Ok(())
    }
}