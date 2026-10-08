use super::{
    StoreError,
    query::{changed_one, read_many, read_one},
};
use crate::types::LanguageScript;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct LanguageScriptStore(Pool<ConnectionManager>);

impl LanguageScriptStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(&self, model: &LanguageScript) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.language_id, &model.script_id]),
        )?;

        Ok(())
    }

    pub fn get(
        &self,
        language_id: &str,
        script_id: &str,
    ) -> Result<Option<LanguageScript>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&language_id, &script_id]),
        )
    }

    pub fn list(&self, language_id: &str) -> Result<Vec<LanguageScript>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&language_id]),
        )
    }

    pub fn delete(&self, language_id: &str, script_id: &str) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        changed_one(connection.execute(
            queries::DELETE,
            Parameters::positional(&[&language_id, &script_id]),
        )?)
    }
}

mod queries {
    pub(super) const INSERT: &str = r#"
        INSERT INTO language_script
        (
            language_id,
            script_id
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
            script_id
        FROM language_script
        WHERE
            language_id = $1
            AND script_id = $2
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            language_id,
            script_id
        FROM language_script
        WHERE
            language_id = $1
        ORDER BY
            script_id
    "#;

    pub(super) const DELETE: &str = r#"
        DELETE
        FROM language_script
        WHERE
            language_id = $1
            AND script_id = $2
    "#;
}
