use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Lexeme, LexemeReading};

pub struct LexemeRepository(Pool<DuckdbConnectionManager>);

impl LexemeRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_id(&self, id: u32) -> Result<Option<Lexeme>> {   
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_LEXEME_BY_ID, [id], Lexeme::from_row)
            .optional()?;

        Ok(entity)
    }

    pub fn get_by_language_id(&self, language_id: u32) -> Result<Box<[Lexeme]>> {   
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_LEXEME_BY_LANGUAGE_ID)?;
        let rows = statement.query_map([language_id], Lexeme::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into_boxed_slice())
    }

    pub fn bulk_insert(&self) -> Result<()> {   
        let connection = self.0.get()?;

        Ok(())
    }
}

pub struct LexemeReadingRepository(Pool<DuckdbConnectionManager>);

impl LexemeReadingRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_by_lexeme_id(&self, id: Uuid) -> Result<Box<[LexemeReading]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_GRAPHEMES_BY_LANGUAGE_SAMPLE)?;
        let rows = statement.query_map([id], LexemeReading::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into_boxed_slice())
    }

    pub fn bulk_insert(&self) -> Result<()> {
        let connection = self.0.get()?;
                
        Ok(())
    }
}