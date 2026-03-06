use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Character};

pub struct CharacterRepository(Pool<DuckdbConnectionManager>);

impl CharacterRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

     pub fn get_sample_by_language(&self, id: Uuid) -> Result<Box<[Character]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_CHARACTERS_BY_LANGUAGE_SAMPLE)?;
        let rows = statement.query_map([id], Character::from_row)?;

        let mut entities = vec![];
        for lang in rows {
            entities.push(lang?);
        }

        Ok(entities.into_boxed_slice())
    }
}