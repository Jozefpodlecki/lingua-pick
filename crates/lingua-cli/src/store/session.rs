use chrono::{DateTime, Utc};
use duckdb_neo::{
    Parameters, r2d2::ConnectionManager, types::TimestampTzValue,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::{store::StoreError, types::Session};

#[derive(Clone)]
pub struct SessionStore(Pool<ConnectionManager>);

impl SessionStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(&self, model: &Session) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            r#"
            INSERT INTO session (
                id,
                user_id,
                target_language_id,
                created_on,
                exercise_count,
                max_exercise_count
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            Parameters::positional(&[
                &model.id,
                &model.user_id,
                &model.target_language_id,
                &model.created_on.to_rfc3339(),
                &model.exercise_count,
                &model.max_exercise_count,
            ]),
        )?;

        Ok(())
    }

    pub fn get_by_id(&self, session_id: Uuid) -> Result<Option<Session>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                id,
                user_id,
                target_language_id,
                created_on,
                exercise_count,
                max_exercise_count
            FROM session
            WHERE id = $1
            "#,
            Parameters::positional(&[&session_id]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? == 0 {
                continue;
            }

            let id = chunk.get_vector_at::<Uuid>(0)?.get(0)?.unwrap();
            let user_id = chunk.get_vector_at::<Uuid>(1)?.get(0)?.unwrap();
            let target_language_id = chunk.get_vector_at::<String>(2)?.get(0)?.unwrap().into();

            let created_on = chunk.get_vector_at::<TimestampTzValue>(3)?.get(0)?.unwrap();
            let created_on = Option::<DateTime<Utc>>::from(*created_on).unwrap();

            let exercise_count = *chunk.get_vector_at::<u16>(4)?.get(0)?.unwrap();
            let max_exercise_count = *chunk.get_vector_at::<u16>(5)?.get(0)?.unwrap();

            return Ok(Some(Session {
                id,
                user_id,
                target_language_id,
                created_on,
                exercise_count,
                max_exercise_count,
            }));
        }

        Ok(None)
    }

    pub fn increment_exercise_count(&self, session_id: Uuid) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            r#"
            UPDATE session
            SET exercise_count = exercise_count + 1
            WHERE id = $1
            "#,
            Parameters::positional(&[&session_id]),
        )?;

        Ok(())
    }
}