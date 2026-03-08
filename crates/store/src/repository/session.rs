use anyhow::Result;
use chrono::Utc;
use duckdb::{params, DuckdbConnectionManager};
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

    pub fn get_by_id(&self, id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        connection.execute(GET_SESSION_BY_ID, [id])?;

        Ok(())
    }
}