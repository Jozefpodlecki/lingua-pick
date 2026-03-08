use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Exercise, ExerciseType};

pub struct ExerciseRepository(Pool<DuckdbConnectionManager>);

impl ExerciseRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<Exercise>> {
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_EXERCISE_BY_ID, [id], Exercise::from_row)
            .optional()?;

        Ok(entity)
    }

    pub fn create(&self, entity: Exercise) -> Result<Exercise> {
        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.created_on.naive_utc()
        ];

        connection.execute(INSERT_EXERCISE, params)?;

        Ok(entity)
    }
}

pub struct ExerciseTypeRepository(Pool<DuckdbConnectionManager>);

impl ExerciseTypeRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_language_id(&self, language_id: Uuid) -> Result<Box<[ExerciseType]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_EXERCISE_TYPES)?;
        let rows = statement.query_map([language_id], ExerciseType::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into_boxed_slice())
    }
}