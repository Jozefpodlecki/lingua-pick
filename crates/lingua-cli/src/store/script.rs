use super::{
    StoreError,
    query::{changed_one, read_many, read_one},
};
use crate::types::Script;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct ScriptStore(Pool<ConnectionManager>);

impl ScriptStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(&self, model: &Script) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.id, &model.name]),
        )?;

        Ok(())
    }

    pub fn update(&self, model: &Script) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[&model.name, &model.id]),
        )?)
    }

    pub fn get(&self, id: &str) -> Result<Option<Script>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(&self) -> Result<Vec<Script>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::None)
    }

    pub fn for_language(&self, language_id: &str) -> Result<Vec<Script>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::FOR_LANGUAGE,
            Parameters::positional(&[&language_id]),
        )
    }
}

mod queries {
    pub(super) const INSERT: &str = r#"
        INSERT INTO script
        (
            id,
            name
        )
        VALUES
        (
            $1,
            $2
        )
    "#;

    pub(super) const UPDATE: &str = r#"
        UPDATE script
        SET
            name = $1
        WHERE
            id = $2
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            id,
            name
        FROM script
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            name
        FROM script
        ORDER BY
            id
    "#;

    pub(super) const FOR_LANGUAGE: &str = r#"
        SELECT
            s.id,
            s.name
        FROM script s
        JOIN language_script l
            ON l.script_id = s.id
        WHERE
            l.language_id = $1
        ORDER BY
            s.id
    "#;
}
