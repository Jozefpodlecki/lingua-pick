use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::ExerciseCategory;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct ExerciseCategoryStore(Pool<ConnectionManager>);

impl ExerciseCategoryStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &ExerciseCategory) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[&model.description, &model.name]),
        )?)
    }

    pub fn insert(&self, model: &ExerciseCategory) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.name, &model.description]),
        )?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Option<ExerciseCategory>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&name]))
    }

    pub fn list(&self) -> Result<Vec<ExerciseCategory>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::positional(&[]))
    }
}

mod queries {
    pub(super) const UPDATE: &str = r#"
        UPDATE exercise_category
        SET
            description = $1
        WHERE
            name = $2
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO exercise_category
        (
            name,
            description
        )
        VALUES
        (
            $1,
            $2
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            name,
            description
        FROM exercise_category
        WHERE
            name = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            name,
            description
        FROM exercise_category
        ORDER BY
            name
    "#;
}
