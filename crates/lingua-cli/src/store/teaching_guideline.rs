use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::TeachingGuideline;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct TeachingGuidelineStore(Pool<ConnectionManager>);

impl TeachingGuidelineStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn list_all(&self) -> Result<Vec<TeachingGuideline>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST_ALL, Parameters::None)
    }

    pub fn update(&self, model: &TeachingGuideline) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[
                &model.category,
                &model.generation_instructions,
                &model.evaluation_instructions,
                &model.code,
            ]),
        )?)
    }

    pub fn insert(&self, model: &TeachingGuideline) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.code,
                &model.category,
                &model.generation_instructions,
                &model.evaluation_instructions,
            ]),
        )?;
        Ok(())
    }

    pub fn get(&self, code: &str) -> Result<Option<TeachingGuideline>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&code]))
    }

    pub fn list(&self, category: &str) -> Result<Vec<TeachingGuideline>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&category]),
        )
    }
}

mod queries {
    pub(super) const LIST_ALL: &str = r#"
        SELECT
            code,
            category,
            generation_instructions,
            evaluation_instructions
        FROM teaching_guideline
        ORDER BY
            code
    "#;

    pub(super) const UPDATE: &str = r#"
        UPDATE teaching_guideline
        SET
            category = $1,
            generation_instructions = $2,
            evaluation_instructions = $3
        WHERE
            code = $4
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO teaching_guideline
        (
            code,
            category,
            generation_instructions,
            evaluation_instructions
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            code,
            category,
            generation_instructions,
            evaluation_instructions
        FROM teaching_guideline
        WHERE
            code = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            code,
            category,
            generation_instructions,
            evaluation_instructions
        FROM teaching_guideline
        WHERE
            category = $1
        ORDER BY
            code
    "#;
}
