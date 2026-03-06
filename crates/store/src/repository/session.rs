use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Session};

pub struct SessionRepository(Pool<DuckdbConnectionManager>);

impl SessionRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, session: Session) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            session.id,
            session.created_on.naive_utc()
        ];

        connection.execute(INSERT_SESSION, params)?;

        Ok(())
    }

    pub fn get(&self, id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        connection.execute(GET_SESSION_BY_ID, [id])?;

        Ok(())
    }
}