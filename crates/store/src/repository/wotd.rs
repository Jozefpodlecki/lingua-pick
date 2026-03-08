use anyhow::Result;
use chrono::{Date, NaiveDate, Utc};
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, WordOfTheDay};

pub struct WordOfTheDayRepository(Pool<DuckdbConnectionManager>);

impl WordOfTheDayRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: WordOfTheDay) -> Result<WordOfTheDay> {
        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.lexeme_id,
            entity.language_id
        ];

        connection.execute(INSERT_WOTD, params)?;

        Ok(entity)
    }

    pub fn get_by_id(&self, id: NaiveDate) -> Result<Option<WordOfTheDay>> {
        let connection = self.0.get()?;

        let entity = connection
            .query_row(GET_WOTD_BY_ID, [id], WordOfTheDay::from_row)
            .optional()?;

        Ok(entity)
    }
}