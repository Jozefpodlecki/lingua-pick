use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Exercise};

pub struct ExerciseRepository(Pool<DuckdbConnectionManager>);

impl ExerciseRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: Exercise) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.created_on.naive_utc()
        ];

        connection.execute(INSERT_EXERCISE, params)?;

        Ok(())
    }
}