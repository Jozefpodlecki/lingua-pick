use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, UserProfile};

pub struct UserProfileRepository(Pool<DuckdbConnectionManager>);

impl UserProfileRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn create(&self, entity: UserProfile) -> Result<()> {

        let connection = self.0.get()?;

        let params = params![
            entity.id,
            entity.user_id,
            entity.created_on,
            entity.updated_on,
            entity.is_active,
            entity.source_language_id,
            entity.target_language_id
        ];

        connection.execute(INSERT_USER_PROFILE, params)?;

        Ok(())
    }

    pub fn set_active(&self, id: Uuid) -> Result<()> {
        let connection = self.0.get()?;

        connection.execute(UPDATE_USER_PROFILE_SET_ACTIVE, [id])?;

        Ok(())
    }

    pub fn get_active_by_user(&self, user_id: Uuid) -> Result<Option<UserProfile>> {
        let connection = self.0.get()?;

        let profile = connection
            .query_row(GET_ACTIVE_USER_PROFILE_BY_USER, [user_id], UserProfile::from_row)
            .optional()?;

        Ok(profile)
    }

    pub fn get_by_user(&self, user_id: Uuid) -> Result<Box<[UserProfile]>> {
        let connection = self.0.get()?;

        let mut statement = connection.prepare(GET_LANGUAGES)?;
        let rows = statement.query_map([], UserProfile::from_row)?;

        let mut entities = vec![];
        for row in rows {
            entities.push(row?);
        }

        Ok(entities.into())
    }
}