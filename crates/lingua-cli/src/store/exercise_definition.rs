use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::{ExerciseDefinition, LanguageExercise, Skill};
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExerciseDefinitionStore(Pool<ConnectionManager>);

impl ExerciseDefinitionStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get_for_language(
        &self,
        language_id: &str,
    ) -> Result<Vec<ExerciseDefinition>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::GET_FOR_LANGUAGE,
            Parameters::positional(&[&language_id]),
        )
    }

    pub fn add_to_language(
        &self,
        language_id: &str,
        definition_id: Uuid,
    ) -> Result<(), StoreError> {
        super::LanguageExerciseStore::new(self.0.clone()).insert(&LanguageExercise {
            language_id: language_id.into(),
            definition_id,
        })
    }

    pub fn add_skill(&self, definition_id: Uuid, skill_id: &str) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::ADD_SKILL,
            Parameters::positional(&[&definition_id, &skill_id]),
        )?;
        Ok(())
    }

    pub fn skills(&self, definition_id: Uuid) -> Result<Vec<Skill>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::SKILLS,
            Parameters::positional(&[&definition_id]),
        )
    }

    pub fn insert(&self, model: &ExerciseDefinition) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let schema = serde_json::to_string(&model.schema)?;
        let answer_schema = model
            .answer_schema
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let verdict_schema = model
            .verdict_schema
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.id,
                &model.kind,
                &model.name,
                &model.description,
                &model.category,
                &model.instructions,
                &schema,
                &answer_schema,
                &verdict_schema,
            ]),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<ExerciseDefinition>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_ID,
            Parameters::positional(&[&id]),
        )
    }

    pub fn list(&self) -> Result<Vec<ExerciseDefinition>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::positional(&[]))
    }

    pub fn get_by_kind(&self, kind: &str) -> Result<Option<ExerciseDefinition>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_KIND,
            Parameters::positional(&[&kind]),
        )
    }
}

mod queries {
    pub(super) const GET_FOR_LANGUAGE: &str = r#"
        SELECT
            d.id,
            d.kind,
            d.name,
            d.description,
            d.category,
            d.instructions,
            d.schema,
            d.answer_schema,
            d.verdict_schema
        FROM exercise_definition d
        JOIN language_exercise l
            ON l.definition_id = d.id
        WHERE
            l.language_id = $1
        ORDER BY
            d.kind
    "#;

    pub(super) const ADD_SKILL: &str = r#"
        INSERT INTO exercise_definition_skill
        (
            definition_id,
            skill_id
        )
        VALUES
        (
            $1,
            $2
        )
    "#;

    pub(super) const SKILLS: &str = r#"
        SELECT
            s.id,
            s.name,
            s.description
        FROM skill s
        JOIN exercise_definition_skill d
            ON d.skill_id = s.id
        WHERE
            d.definition_id = $1
        ORDER BY
            s.id
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO exercise_definition
        (
            id,
            kind,
            name,
            description,
            category,
            instructions,
            schema,
            answer_schema,
            verdict_schema
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9
        )
    "#;

    pub(super) const GET_BY_ID: &str = r#"
        SELECT
            id,
            kind,
            name,
            description,
            category,
            instructions,
            schema,
            answer_schema,
            verdict_schema
        FROM exercise_definition
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            kind,
            name,
            description,
            category,
            instructions,
            schema,
            answer_schema,
            verdict_schema
        FROM exercise_definition
        ORDER BY
            kind
    "#;

    pub(super) const GET_BY_KIND: &str = r#"
        SELECT
            id,
            kind,
            name,
            description,
            category,
            instructions,
            schema,
            answer_schema,
            verdict_schema
        FROM exercise_definition
        WHERE
            kind = $1
    "#;
}
