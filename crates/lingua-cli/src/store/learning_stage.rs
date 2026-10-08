use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::{ExerciseDefinition, LearningStage, StageExercise, StageSkill};
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct LearningStageStore(Pool<ConnectionManager>);

impl LearningStageStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &LearningStage) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let existing: LearningStage = read_one(
            &connection,
            queries::GET_SEQUENCE,
            Parameters::positional(&[&model.code]),
        )?
        .ok_or(StoreError::NotFound)?;
        if model.sequence != existing.sequence {
            return Err(StoreError::InvalidInput(
                "stage order is immutable; update teaching instructions separately",
            ));
        }
        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[
                &model.name,
                &model.generation_instructions,
                &model.evaluation_instructions,
                &model.code,
            ]),
        )?)
    }

    pub fn add_dependency(
        &self,
        stage_code: &str,
        prerequisite_code: &str,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::add_dependency(
            &connection,
            "learning_stage_dependency",
            "stage_code",
            "prerequisite_code",
            stage_code,
            prerequisite_code,
        )
    }

    pub fn prerequisites(&self, stage_code: &str) -> Result<Vec<LearningStage>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::PREREQUISITES,
            Parameters::positional(&[&stage_code]),
        )
    }

    pub fn definitions(&self, stage_code: &str) -> Result<Vec<ExerciseDefinition>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::DEFINITIONS,
            Parameters::positional(&[&stage_code]),
        )
    }

    pub fn insert(&self, model: &LearningStage) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.code,
                &model.sequence,
                &model.name,
                &model.generation_instructions,
                &model.evaluation_instructions,
            ]),
        )?;
        Ok(())
    }

    pub fn get(&self, code: &str) -> Result<Option<LearningStage>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&code]))
    }

    pub fn list(&self) -> Result<Vec<LearningStage>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::positional(&[]))
    }

    pub fn add_skill(&self, model: &StageSkill) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let evidence_mode = model.evidence_mode.as_str();
        connection.execute(
            queries::ADD_SKILL,
            Parameters::positional(&[&model.stage_code, &model.skill_id, &evidence_mode]),
        )?;
        Ok(())
    }

    pub fn list_skills(&self, stage_code: &str) -> Result<Vec<StageSkill>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST_SKILLS,
            Parameters::positional(&[&stage_code]),
        )
    }

    pub fn add_exercise(&self, model: &StageExercise) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::ADD_EXERCISE,
            Parameters::positional(&[
                &model.stage_code,
                &model.definition_id,
                &model.selection_instructions,
            ]),
        )?;
        Ok(())
    }

    pub fn list_exercises(&self, stage_code: &str) -> Result<Vec<StageExercise>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST_EXERCISES,
            Parameters::positional(&[&stage_code]),
        )
    }
}

mod queries {
    pub(super) const GET_SEQUENCE: &str = r#"
        SELECT
            code,
            sequence,
            name,
            generation_instructions,
            evaluation_instructions
        FROM learning_stage
        WHERE
            code = $1
    "#;

    pub(super) const UPDATE: &str = r#"
        GET_SEQUENCE learning_stage
        SET
            name = $1,
            generation_instructions = $2,
            evaluation_instructions = $3
        WHERE
            code = $4
    "#;

    pub(super) const PREREQUISITES: &str = r#"
        SELECT
            s.code,
            s.sequence,
            s.name,
            s.generation_instructions,
            s.evaluation_instructions
        FROM learning_stage s
        JOIN learning_stage_dependency d
            ON d.prerequisite_code = s.code
        WHERE
            d.stage_code = $1
        ORDER BY
            s.sequence
    "#;

    pub(super) const DEFINITIONS: &str = r#"
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
        JOIN learning_stage_exercise s
            ON s.definition_id = d.id
        WHERE
            s.stage_code = $1
        ORDER BY
            d.kind
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO learning_stage
        (
            code,
            sequence,
            name,
            generation_instructions,
            evaluation_instructions
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            code,
            sequence,
            name,
            generation_instructions,
            evaluation_instructions
        FROM learning_stage
        WHERE
            code = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            code,
            sequence,
            name,
            generation_instructions,
            evaluation_instructions
        FROM learning_stage
        ORDER BY
            sequence
    "#;

    pub(super) const ADD_SKILL: &str = r#"
        INSERT INTO learning_stage_skill
        (
            stage_code,
            skill_id,
            evidence_mode
        )
        VALUES
        (
            $1,
            $2,
            $3
        )
    "#;

    pub(super) const LIST_SKILLS: &str = r#"
        SELECT
            stage_code,
            skill_id,
            evidence_mode
        FROM learning_stage_skill
        WHERE
            stage_code = $1
        ORDER BY
            skill_id
    "#;

    pub(super) const ADD_EXERCISE: &str = r#"
        INSERT INTO learning_stage_exercise
        (
            stage_code,
            definition_id,
            selection_instructions
        )
        VALUES
        (
            $1,
            $2,
            $3
        )
    "#;

    pub(super) const LIST_EXERCISES: &str = r#"
        SELECT
            stage_code,
            definition_id,
            selection_instructions
        FROM learning_stage_exercise
        WHERE
            stage_code = $1
        ORDER BY
            definition_id
    "#;
}
