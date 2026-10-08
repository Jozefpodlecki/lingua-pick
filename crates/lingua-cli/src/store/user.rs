use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::User;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserStore(Pool<ConnectionManager>);

impl UserStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn update(&self, model: &User) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let updated_on = model.updated_on.to_rfc3339();

        super::query::changed_one(connection.execute(
            queries::UPDATE,
            Parameters::positional(&[
                &model.source_language_id,
                &model.username,
                &model.password_hash,
                &updated_on,
                &model.id,
            ]),
        )?)
    }

    pub fn insert(&self, model: &User) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let created_on = model.created_on.to_rfc3339();
        let updated_on = model.updated_on.to_rfc3339();
        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.id,
                &model.source_language_id,
                &model.username,
                &model.password_hash,
                &created_on,
                &updated_on,
            ]),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<User>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET_BY_ID,
            Parameters::positional(&[&id]),
        )
    }

    pub fn list(&self) -> Result<Vec<User>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::positional(&[]))
    }
}

mod queries {
    pub(super) const UPDATE: &str = r#"
        UPDATE user
        SET
            source_language_id = $1,
            username = $2,
            password_hash = $3,
            updated_on = $4
        WHERE
            id = $5
    "#;

    pub(super) const INSERT: &str = r#"
        INSERT INTO user
        (
            id,
            source_language_id,
            username,
            password_hash,
            created_on,
            updated_on
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

    pub(super) const GET_BY_ID: &str = r#"
        SELECT
            id,
            source_language_id,
            username,
            password_hash,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(updated_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS updated_on
        FROM user
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            source_language_id,
            username,
            password_hash,
            strftime(created_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS created_on,
            strftime(updated_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS updated_on
        FROM user
        ORDER BY
            username,
            id
    "#;
}
