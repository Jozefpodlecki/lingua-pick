use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::{Topic, TopicStage};
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct TopicStore(Pool<ConnectionManager>);

impl TopicStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &Topic) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[&model.name, &model.description, &model.id]),
        )?)
    }

    pub fn insert(&self, model: &Topic) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::INSERT,
            Parameters::positional(&[&model.id, &model.name, &model.description]),
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<Topic>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(&self) -> Result<Vec<Topic>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::positional(&[]))
    }

    pub fn add_to_stage(&self, model: &TopicStage) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            queries::ADD_TO_STAGE,
            Parameters::positional(&[
                &model.topic_id,
                &model.stage_code,
                &model.generation_instructions,
            ]),
        )?;
        Ok(())
    }

    pub fn list_for_stage(&self, stage_code: &str) -> Result<Vec<TopicStage>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST_FOR_STAGE,
            Parameters::positional(&[&stage_code]),
        )
    }
}

mod queries {
    pub(super) const UPDATE: &str = r#"
        UPDATE topic
        SET
            name = $1,
            description = $2
        WHERE
            id = $3
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO topic
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
        FROM topic
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            name,
            description
        FROM topic
        ORDER BY
            id
    "#;

    pub(super) const ADD_TO_STAGE: &str = r#"
        INSERT INTO topic_stage
        (
            topic_id,
            stage_code,
            generation_instructions
        )
        VALUES
        (
            $1,
            $2,
            $3
        )
    "#;

    pub(super) const LIST_FOR_STAGE: &str = r#"
        SELECT
            topic_id,
            stage_code,
            generation_instructions
        FROM topic_stage
        WHERE
            stage_code = $1
        ORDER BY
            topic_id
    "#;
}
