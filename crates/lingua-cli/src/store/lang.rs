use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::Language;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct LanguageStore(Pool<ConnectionManager>);

impl LanguageStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &Language) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[&model.name, &model.region, &model.native_name, &model.id]),
        )?)
    }

    pub fn insert_json(&self, path: &str) -> Result<(), StoreError> {
        let contents = std::fs::read_to_string(path)?;
        let languages: Vec<Language> = serde_json::from_str(&contents)?;
        let connection = self.0.get()?;

        super::query::transaction(&connection, || {
            for language in &languages {
                connection.execute(
                    queries::INSERT_JSON,
                    Parameters::positional(&[
                        &language.id,
                        &language.name,
                        &language.region,
                        &language.native_name,
                    ]),
                )?;
            }

            Ok(())
        })
    }

    pub fn insert(&self, model: &Language) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.id, &model.name, &model.region, &model.native_name]),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Language>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_ID,
            Parameters::positional(&[&id]),
        )
    }

    pub fn get_all(&self) -> Result<Vec<Language>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::GET_ALL, Parameters::positional(&[]))
    }
}

mod queries {
    pub(super) const UPDATE: &str = r#"
        UPDATE language
        SET
            name = $1,
            region = $2,
            native_name = $3
        WHERE
            id = $4
    "#;

    pub(super) const INSERT_JSON: &str = r#"
        INSERT INTO language
        (
            id,
            name,
            region,
            native_name
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4
        )
        ON CONFLICT (id) DO NOTHING
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO language
        (
            id,
            name,
            region,
            native_name
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4
        )
    "#;

    pub(super) const GET_BY_ID: &str = r#"
        SELECT
            id,
            name,
            region,
            native_name
        FROM language
        WHERE
            id = $1
    "#;

    pub(super) const GET_ALL: &str = r#"
        SELECT
            id,
            name,
            region,
            native_name
        FROM language
        ORDER BY
            id
    "#;
}
