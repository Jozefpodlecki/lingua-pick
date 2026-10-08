use duckdb_neo::{
    Parameters, r2d2::ConnectionManager,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::{store::StoreError, types::ExerciseDefinition};

#[derive(Clone)]
pub struct ExerciseDefinitionStore(Pool<ConnectionManager>);

impl ExerciseDefinitionStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(&self, model: &ExerciseDefinition) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let schema = serde_json::to_string(&model.schema)?;
        let answer_schema = model.answer_schema.as_ref().map(serde_json::to_string).transpose()?;
        let verdict_schema = model.verdict_schema.as_ref().map(serde_json::to_string).transpose()?;

        connection.execute(
            r#"
            INSERT INTO exercise_definition (
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
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
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

        let result = connection.query(
            r#"
            SELECT
                id,
                kind,
                name,
                description,
                category,
                instructions,
                schema::VARCHAR,
                answer_schema::VARCHAR,
                verdict_schema::VARCHAR
            FROM exercise_definition
            WHERE id = $1
            "#,
            Parameters::positional(&[&id]),
        )?;

        for chunk in result {
            let chunk = chunk?;
            if chunk.row_count()? == 0 { continue; }

            let id = chunk.get_vector_at::<Uuid>(0)?.get(0)?.unwrap();
            let kind = chunk.get_vector_at::<String>(1)?.get(0)?.unwrap().into();
            let name = chunk.get_vector_at::<String>(2)?.get(0)?.unwrap().into();
            let description = chunk.get_vector_at::<String>(3)?.get(0)?.unwrap().into();
            let category = chunk.get_vector_at::<String>(4)?.get(0)?.unwrap().into();
            let instructions = chunk.get_vector_at::<String>(5)?.get(0)?.unwrap().into();

            let schema = chunk.get_vector_at::<String>(6)?.get(0)?.unwrap();
            let schema = serde_json::from_str(schema)?;

            let answer_schema = chunk.get_vector_at::<String>(7)?.get(0)?.map(serde_json::from_str).transpose()?;
            let verdict_schema = chunk.get_vector_at::<String>(8)?.get(0)?.map(serde_json::from_str).transpose()?;

            return Ok(Some(ExerciseDefinition {
                id,
                kind,
                name,
                description,
                category,
                instructions,
                schema,
                answer_schema,
                verdict_schema,
            }));
        }

        Ok(None)
    }

    pub fn get_for_language(&self, language_id: &str) -> Result<Vec<ExerciseDefinition>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                d.id,
                d.kind,
                d.name,
                d.description,
                d.category,
                d.instructions,
                d.schema::VARCHAR,
                d.answer_schema::VARCHAR,
                d.verdict_schema::VARCHAR
            FROM exercise_definition d
            INNER JOIN language_exercise le
                ON le.definition_id = d.id
            WHERE le.language_id = $1
            ORDER BY d.kind
            "#,
            Parameters::positional(&[&language_id]),
        )?;

        let mut definitions = Vec::new();

        for chunk in result {
            let chunk = chunk?;

            for row in 0..chunk.row_count()? {
                let id = chunk.get_vector_at::<Uuid>(0)?.get(row)?.unwrap();
                let kind = chunk.get_vector_at::<String>(1)?.get(row)?.unwrap().into();
                let name = chunk.get_vector_at::<String>(2)?.get(row)?.unwrap().into();
                let description = chunk.get_vector_at::<String>(3)?.get(row)?.unwrap().into();
                let category = chunk.get_vector_at::<String>(4)?.get(row)?.unwrap().into();
                let instructions = chunk.get_vector_at::<String>(5)?.get(row)?.unwrap().into();

                let schema = chunk.get_vector_at::<String>(6)?.get(row)?.unwrap();
                let schema = serde_json::from_str(schema)?;

                let answer_schema = chunk.get_vector_at::<String>(7)?.get(row)?.map(serde_json::from_str).transpose()?;
                let verdict_schema = chunk.get_vector_at::<String>(8)?.get(row)?.map(serde_json::from_str).transpose()?;

                definitions.push(ExerciseDefinition {
                    id,
                    kind,
                    name,
                    description,
                    category,
                    instructions,
                    schema,
                    answer_schema,
                    verdict_schema,
                });
            }
        }

        Ok(definitions)
    }

    pub fn add_to_language(
        &self,
        language_id: &str,
        definition_id: Uuid,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            r#"
            INSERT INTO language_exercise (
                language_id,
                definition_id
            )
            VALUES ($1, $2)
            "#,
            Parameters::positional(&[
                &language_id,
                &definition_id,
            ]),
        )?;

        Ok(())
    }
}