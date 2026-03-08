use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Grapheme, GraphemeReading};

pub struct GraphemeRepository(Pool<DuckdbConnectionManager>);

impl GraphemeRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_id(&self, id: u32) -> Result<Option<Grapheme>> {
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_GRAPHEME_BY_ID, [id], Grapheme::from_row)
            .optional()?;

        Ok(entity)
    }

    pub fn get_sample_by_language(&self, id: u32) -> Result<Box<[Grapheme]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_GRAPHEMES_BY_LANGUAGE_SAMPLE)?;
        let rows = statement.query_map([id], Grapheme::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into_boxed_slice())
    }
}

pub struct GraphemeReadingRepository(Pool<DuckdbConnectionManager>);

impl GraphemeReadingRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_grapheme_id(&self, id: Uuid) -> Result<Box<[GraphemeReading]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_GRAPHEMES_BY_LANGUAGE_SAMPLE)?;
        let rows = statement.query_map([id], GraphemeReading::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into_boxed_slice())
    }
}