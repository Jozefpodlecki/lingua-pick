use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::UserStats;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserStatsStore(Pool<ConnectionManager>);

impl UserStatsStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &UserStats) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let updated_on = model.updated_on.to_rfc3339();
        let categories = serde_json::to_string(&model.struggling_categories)?;

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[
                &updated_on,
                &categories,
                &model.user_id,
                &model.target_language_id,
            ]),
        )?)
    }

    pub fn insert(&self, model: &UserStats) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let created_on = model.created_on.to_rfc3339();
        let updated_on = model.updated_on.to_rfc3339();
        let struggling_categories = serde_json::to_string(&model.struggling_categories)?;
        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.user_id,
                &model.target_language_id,
                &created_on,
                &updated_on,
                &struggling_categories,
            ]),
        )?;
        Ok(())
    }

    pub fn get(
        &self,
        user_id: Uuid,
        target_language_id: &str,
    ) -> Result<Option<UserStats>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&user_id, &target_language_id]),
        )
    }

    pub fn list(&self, user_id: Uuid) -> Result<Vec<UserStats>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&user_id]),
        )
    }
}

mod queries {
    pub(super) const UPDATE: &str = r#"
        UPDATE user_stats
        SET
            updated_on = $1,
            struggling_categories = $2
        WHERE
            user_id = $3
            AND target_language_id = $4
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO user_stats
        (
            user_id,
            target_language_id,
            created_on,
            updated_on,
            struggling_categories
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
            user_id,
            target_language_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(updated_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS updated_on,
            struggling_categories
        FROM user_stats
        WHERE
            user_id = $1
            AND target_language_id = $2
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            user_id,
            target_language_id,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(updated_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS updated_on,
            struggling_categories
        FROM user_stats
        WHERE
            user_id = $1
        ORDER BY
            target_language_id
    "#;
}
