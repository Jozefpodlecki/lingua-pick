use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::Session;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionStore(Pool<ConnectionManager>);

impl SessionStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn refresh_progress(&self, session_id: Uuid) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        refresh_progress(&connection, session_id)
    }

    pub fn insert(&self, model: &Session) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let created_on = model.created_on.to_rfc3339();
        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.id,
                &model.user_id,
                &model.target_language_id,
                &created_on,
                &model.exercise_count,
                &model.max_exercise_count,
                &model.last_exercise_id,
            ]),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<Session>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_ID,
            Parameters::positional(&[&id]),
        )
    }

    pub fn list(
        &self,
        user_id: Uuid,
        target_language_id: &str,
    ) -> Result<Vec<Session>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&user_id, &target_language_id]),
        )
    }
}

pub(super) fn refresh_progress(
    connection: &duckdb_neo::connection::Connection,
    session_id: Uuid,
) -> Result<(), StoreError> {
    super::query::changed_one(connection.execute(
        queries::REFRESH_PROGRESS,
        Parameters::positional(&[&session_id]),
    )?)
}

mod queries {
    pub(super) const INSERT: &str = r#"
        INSERT INTO session
        (
            id,
            user_id,
            target_language_id,
            created_on,
            exercise_count,
            max_exercise_count,
            last_exercise_id
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7
        )
    "#;

    pub(super) const GET_BY_ID: &str = r#"
        SELECT
            id,
            user_id,
            target_language_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            exercise_count,
            max_exercise_count,
            last_exercise_id
        FROM session
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            user_id,
            target_language_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            exercise_count,
            max_exercise_count,
            last_exercise_id
        FROM session
        WHERE
            user_id = $1
            AND target_language_id = $2
        ORDER BY
            created_on,
            id
    "#;

    pub(super) const REFRESH_PROGRESS: &str = r#"
        UPDATE session
        SET
            exercise_count =
            (
                SELECT
                    count(*)
                FROM exercise
                WHERE
                    session_id = $1
                    AND answered_on IS NOT NULL
            ),
            last_exercise_id =
            (
                SELECT
                    id
                FROM exercise
                WHERE
                    session_id = $1
                    AND answered_on IS NOT NULL
                ORDER BY
                    answered_on DESC,
                    id DESC
                LIMIT 1
            )
        WHERE
            id = $1
    "#;
}
