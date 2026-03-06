use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Language};

pub struct LanguageRepository(Pool<DuckdbConnectionManager>);

impl LanguageRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_all(&self) -> Result<Box<[Language]>> {
        let connection = self.0.get()?;

        let count: usize = connection.query_row(GET_LANGUAGES_COUNT, [], |row| row.get(0))?;

        let mut statement = connection.prepare(GET_LANGUAGES)?;
        let rows = statement.query_map([], Language::from_row)?;

        let mut languages = Vec::with_capacity(count);
        for row in rows {
            languages.push(row?);
        }

        Ok(languages.into_boxed_slice())
    }
}

