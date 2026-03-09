use anyhow::Result;
use chrono::{DateTime, Utc};
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Session};

pub struct SessionRepository(Pool<DuckdbConnectionManager>);

impl SessionRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: Session) -> Result<Session> {
        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.user_profile_id,
            entity.created_on,
            entity.updated_on,
            entity.completed_on,
            entity.exercise_count,
            entity.total_exercise_count,
            entity.current_exercise_id,
        ];

        connection.execute(INSERT_SESSION, params)?;

        Ok(entity)
    }

    pub fn link_to_user(&self, id: Uuid, user_id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            id,
            user_id,
            Utc::now()
        ];

        connection.execute(INSERT_SESSION_USER, params)?;

        Ok(())
    }

    pub fn set_exercise(&self, id: Uuid, exercise_id: u32) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            exercise_id,
            id
        ];

        connection.execute(UPDATE_SESSION_SET_EXERCISE, params)?;

        Ok(())
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<Session>> {
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_SESSION_BY_ID, [id], Session::from_row)
            .optional()?;

        Ok(entity)
    }
}