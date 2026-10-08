use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::LanguageExercise;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct LanguageExerciseStore(Pool<ConnectionManager>);

impl LanguageExerciseStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn delete(&self, language_id: &str, definition_id: Uuid) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::changed_one(connection.execute(
            queries::DELETE,
            Parameters::positional(&[&language_id, &definition_id]),
        )?)
    }

    pub fn exists(&self, language_id: &str, definition_id: Uuid) -> Result<bool, StoreError> {
        Ok(self.get(language_id, definition_id)?.is_some())
    }

    pub fn get_definition_ids(&self, language_id: &str) -> Result<Vec<Uuid>, StoreError> {
        Ok(self
            .list(language_id)?
            .into_iter()
            .map(|record| record.definition_id)
            .collect())
    }

    pub fn insert(&self, model: &LanguageExercise) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.language_id, &model.definition_id]),
        )?;
        Ok(())
    }

    pub fn get(
        &self,
        language_id: &str,
        definition_id: Uuid,
    ) -> Result<Option<LanguageExercise>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&language_id, &definition_id]),
        )
    }

    pub fn list(&self, language_id: &str) -> Result<Vec<LanguageExercise>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&language_id]),
        )
    }
}

mod queries {
    pub(super) const DELETE: &str = r#"
        DELETE
        FROM language_exercise
        WHERE
            language_id = $1
            AND definition_id = $2
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO language_exercise
        (
            language_id,
            definition_id
        )
        VALUES
        (
            $1,
            $2
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            language_id,
            definition_id
        FROM language_exercise
        WHERE
            language_id = $1
            AND definition_id = $2
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            language_id,
            definition_id
        FROM language_exercise
        WHERE
            language_id = $1
        ORDER BY
            definition_id
    "#;
}
