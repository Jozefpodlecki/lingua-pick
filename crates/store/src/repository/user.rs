use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, User};

pub struct UserRepository(Pool<DuckdbConnectionManager>);

impl UserRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: User) -> Result<User> {

        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.created_on.naive_utc(),
            entity.user_name,
            entity.password_hash
        ];

        connection.execute(INSERT_USER, params)?;

        Ok(entity)
    }

    pub fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        
        let connection = self.0.get()?;

        let result = connection
            .query_row(GET_USER_BY_USERNAME, [username], User::from_row)
            .optional()?;

        Ok(result)
    }
}
