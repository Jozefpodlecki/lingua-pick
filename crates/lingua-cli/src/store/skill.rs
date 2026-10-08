use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::Skill;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct SkillStore(Pool<ConnectionManager>);

impl SkillStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &Skill) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[&model.name, &model.description, &model.id]),
        )?)
    }

    pub fn add_dependency(&self, skill_id: &str, prerequisite_id: &str) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::add_dependency(
            &connection,
            "skill_dependency",
            "skill_id",
            "prerequisite_id",
            skill_id,
            prerequisite_id,
        )
    }

    pub fn prerequisites(&self, skill_id: &str) -> Result<Vec<Skill>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::PREREQUISITES,
            Parameters::positional(&[&skill_id]),
        )
    }

    pub fn insert(&self, model: &Skill) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.id, &model.name, &model.description]),
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<Skill>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(&self) -> Result<Vec<Skill>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::positional(&[]))
    }
}

mod queries {
    pub(super) const UPDATE: &str = r#"
        UPDATE skill
        SET
            name = $1,
            description = $2
        WHERE
            id = $3
    "#;

    pub(super) const PREREQUISITES: &str = r#"
        SELECT
            s.id,
            s.name,
            s.description
        FROM skill s
        JOIN skill_dependency d
            ON d.prerequisite_id = s.id
        WHERE
            d.skill_id = $1
        ORDER BY
            s.id
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO skill
        (
            id,
            name,
            description
        )
        VALUES
        (
            $1,
            $2,
            $3
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            id,
            name,
            description
        FROM skill
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            name,
            description
        FROM skill
        ORDER BY
            id
    "#;
}
