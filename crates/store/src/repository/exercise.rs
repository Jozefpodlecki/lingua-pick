use anyhow::Result;
use chrono::{DateTime, Utc};
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Exercise, ExerciseType};

pub struct ExerciseRepository(Pool<DuckdbConnectionManager>);

impl ExerciseRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_id(&self, id: u32) -> Result<Option<Exercise>> {
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_EXERCISE_BY_ID, [id], Exercise::from_row)
            .optional()?;

        Ok(entity)
    }

    pub fn create(&self, entity: &Exercise) -> Result<u32> {
        let connection = self.0.get()?;

        let params = params![
            entity.type_id,
            entity.session_id,
            entity.created_on,
            entity.completed_on,
            entity.data,
            entity.result,
            entity.verdict
        ];

        let mut statement = connection.prepare(INSERT_EXERCISE)?;
        let id: u32 = statement.query_row(params, |row| row.get(0))?;

        Ok(id)
    }

    pub fn set_data(&self, id: u32, data: &str) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            data,
            id
        ];

        connection.execute(UPDATE_EXERCISE_SET_DATA, params)?;

        Ok(())
    }

    pub fn set_result(&self, id: u32, result: &str) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            result,
            id
        ];

        connection.execute(UPDATE_EXERCISE_SET_RESULT, params)?;

        Ok(())
    }

    pub fn set_verdict(&self, id: u32, verdict: &str) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            verdict,
            id
        ];

        connection.execute(UPDATE_EXERCISE_SET_VERDICT, params)?;

        Ok(())
    }

    pub fn complete(&self, id: u32, completed_on: DateTime<Utc>) -> Result<()> {
        let connection = self.0.get()?;

        let params = params![
            completed_on,
            id
        ];

        connection.execute(UPDATE_EXERCISE_SET_COMPLETED, params)?;

        Ok(())
    }

}

pub struct ExerciseTypeRepository(Pool<DuckdbConnectionManager>);

impl ExerciseTypeRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_name(&self, name: &str) -> Result<Option<ExerciseType>> {
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_EXERCISE_TYPE_BY_NAME, [name], ExerciseType::from_row)
            .optional()?;

        Ok(entity)
    }

    pub fn get_by_language_id(&self, language_id: u32) -> Result<Box<[ExerciseType]>> {
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