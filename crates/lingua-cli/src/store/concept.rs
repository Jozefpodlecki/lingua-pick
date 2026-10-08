use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::Concept;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ConceptStore(Pool<ConnectionManager>);

impl ConceptStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn search(
        &self,
        language_id: &str,
        query: &str,
        skill_id: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Concept>, StoreError> {
        if limit == 0 || limit > 20 || offset > 10_000 || query.len() > 256 {
            return Err(StoreError::InvalidInput("invalid concept search bounds"));
        }
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::SEARCH,
            Parameters::positional(&[&language_id, &query, &skill_id, &limit, &offset]),
        )
    }

    pub fn get_by_code(
        &self,
        language_id: &str,
        code: &str,
    ) -> Result<Option<Concept>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_CODE,
            Parameters::positional(&[&language_id, &code]),
        )
    }

    pub fn add_dependency(
        &self,
        concept_id: Uuid,
        prerequisite_id: Uuid,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        if !super::query::has_rows(
            &connection,
            queries::ADD_DEPENDENCY,
            Parameters::positional(&[&concept_id, &prerequisite_id]),
        )? {
            return Err(StoreError::InvalidInput(
                "concept prerequisites must belong to the same target",
            ));
        }
        super::query::add_dependency(
            &connection,
            "concept_dependency",
            "concept_id",
            "prerequisite_id",
            &concept_id.to_string(),
            &prerequisite_id.to_string(),
        )
    }

    pub fn prerequisite_ids(&self, concept_id: Uuid) -> Result<Vec<Uuid>, StoreError> {
        let connection = self.0.get()?;

        super::query::read_ids(
            &connection,
            queries::PREREQUISITE_IDS,
            Parameters::positional(&[&concept_id]),
        )
    }

    pub fn insert(&self, model: &Concept) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::transaction(&connection, || {
            connection.execute(
                queries::INSERT,
                Parameters::positional(&[
                    &model.id,
                    &model.language_id,
                    &model.skill_id,
                    &model.code,
                    &model.name,
                    &model.description,
                ]),
            )?;
            connection.execute(
                queries::INSERT_LANGUAGE_ASSOCIATION,
                Parameters::positional(&[&model.language_id, &model.id]),
            )?;
            Ok(())
        })
    }

    pub fn get(&self, id: Uuid) -> Result<Option<Concept>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(&self, language_id: &str) -> Result<Vec<Concept>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&language_id]),
        )
    }
}

mod queries {
    pub(super) const SEARCH: &str = r#"
        SELECT
            id,
            language_id,
            skill_id,
            code,
            name,
            description
        FROM concept
        WHERE
            language_id = $1
            AND ($3 IS NULL OR skill_id = $3)
            AND
            (
                contains(lower(code), lower($2))
                OR contains(lower(name), lower($2))
                OR contains(lower(description), lower($2))
            )
        ORDER BY
            code
        LIMIT $4
        OFFSET $5
    "#;

    pub(super) const GET_BY_CODE: &str = r#"
        SELECT
            id,
            language_id,
            skill_id,
            code,
            name,
            description
        FROM concept
        WHERE
            language_id = $1
            AND code = $2
    "#;

    pub(super) const ADD_DEPENDENCY: &str = r#"
        SELECT
            1
        FROM concept a
        JOIN concept b
            ON a.language_id = b.language_id
        WHERE
            a.id = $1
            AND b.id = $2
    "#;

    pub(super) const PREREQUISITE_IDS: &str = r#"
        SELECT
            prerequisite_id AS id
        FROM concept_dependency
        WHERE
            concept_id = $1
        ORDER BY
            prerequisite_id
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO concept
        (
            id,
            language_id,
            skill_id,
            code,
            name,
            description
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        )
    "#;

    pub(super) const INSERT_LANGUAGE_ASSOCIATION: &str = r#"
        INSERT INTO language_concept
        (
            language_id,
            concept_id
        )
        VALUES
        (
            $1,
            $2
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            id,
            language_id,
            skill_id,
            code,
            name,
            description
        FROM concept
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            language_id,
            skill_id,
            code,
            name,
            description
        FROM concept
        WHERE
            language_id = $1
        ORDER BY
            code
    "#;
}
