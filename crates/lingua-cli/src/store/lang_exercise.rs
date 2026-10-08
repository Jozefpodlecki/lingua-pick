use duckdb_neo::{
    Parameters, r2d2::ConnectionManager,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::store::StoreError;

#[derive(Clone)]
pub struct LanguageExerciseStore(Pool<ConnectionManager>);

impl LanguageExerciseStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(
        &self,
        language_id: &str,
        definition_id: Uuid,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            r#"
            INSERT INTO language_exercise (
                language_id,
                definition_id
            )
            VALUES ($1, $2)
            "#,
            Parameters::positional(&[
                &language_id,
                &definition_id,
            ]),
        )?;

        Ok(())
    }

    pub fn delete(
        &self,
        language_id: &str,
        definition_id: Uuid,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            r#"
            DELETE FROM language_exercise
            WHERE language_id = $1
              AND definition_id = $2
            "#,
            Parameters::positional(&[
                &language_id,
                &definition_id,
            ]),
        )?;

        Ok(())
    }

    pub fn exists(
        &self,
        language_id: &str,
        definition_id: Uuid,
    ) -> Result<bool, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT definition_id
            FROM language_exercise
            WHERE language_id = $1
              AND definition_id = $2
            LIMIT 1
            "#,
            Parameters::positional(&[
                &language_id,
                &definition_id,
            ]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? > 0 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn get_definition_ids(
        &self,
        language_id: &str,
    ) -> Result<Vec<Uuid>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT definition_id
            FROM language_exercise
            WHERE language_id = $1
            "#,
            Parameters::positional(&[&language_id]),
        )?;

        let mut ids = Vec::new();

        for chunk in result {
            let chunk = chunk?;

            for row in 0..chunk.row_count()? {
                let id = chunk.get_vector_at::<Uuid>(0)?.get(row)?.unwrap();
                ids.push(id);
            }
        }

        Ok(ids)
    }
}