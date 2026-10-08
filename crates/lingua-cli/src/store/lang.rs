use chrono::{DateTime, Utc};
use duckdb_neo::{
    Parameters, r2d2::ConnectionManager, types::TimestampTzValue,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::{store::StoreError, types::Language};

#[derive(Clone)]
pub struct LanguageStore(Pool<ConnectionManager>);

impl LanguageStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert_json(&self, path: &str) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            format!(r#"
            INSERT INTO language
            SELECT *
            FROM read_json(
                '{path}',
                columns = {{
                    id: 'VARCHAR',
                    name: 'VARCHAR',
                    region: 'VARCHAR',
                    native_name: 'VARCHAR'
                }}
            )
            "#).as_str(),
            Parameters::None,
        )?;

        Ok(())
    }

    pub fn get_by_id(&self, language_id: &str) -> Result<Option<Language>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                id,
                name,
                region,
                native_name
            FROM language
            WHERE id = $1
            "#,
            Parameters::positional(&[&language_id]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? == 0 {
                continue;
            }

            let id = chunk.get_vector_at::<String>(0)?.get(0)?.unwrap().into();
            let name = chunk.get_vector_at::<String>(1)?.get(0)?.unwrap().into();
            let region = chunk.get_vector_at::<String>(2)?.get(0)?.unwrap().into();
            let native_name = chunk.get_vector_at::<String>(3)?.get(0)?.unwrap().into();

            return Ok(Some(Language {
                id,
                name,
                region,
                native_name,
            }));
        }

        Ok(None)
    }

    pub fn get_all(&self) -> Result<Vec<Language>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                id,
                name,
                region,
                native_name
            FROM language
            ORDER BY name
            "#,
            Parameters::None,
        )?;

        let mut languages = Vec::new();

        for chunk in result {
            let chunk = chunk?;

            let ids = chunk.get_vector_at::<String>(0)?;
            let names = chunk.get_vector_at::<String>(1)?;
            let regions = chunk.get_vector_at::<String>(2)?;
            let native_names = chunk.get_vector_at::<String>(3)?;

            for row in 0..chunk.row_count()? {
                languages.push(Language {
                    id: ids.get(row)?.unwrap().into(),
                    name: names.get(row)?.unwrap().into(),
                    region: regions.get(row)?.unwrap().into(),
                    native_name: native_names.get(row)?.unwrap().into(),
                });
            }
        }

        Ok(languages)
    }
}