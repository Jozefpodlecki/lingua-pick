use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, LanguageAsset};

pub struct LanguageAssetsRepository(Pool<DuckdbConnectionManager>);

impl LanguageAssetsRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

     pub fn get_by_language_id(&self, id: u32) -> Result<Box<[LanguageAsset]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_ASSETS_BY_LANGUAGE_ID)?;
        let rows = statement.query_map([id], LanguageAsset::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into_boxed_slice())
    }
}