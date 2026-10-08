use duckdb_neo::{
    Parameters, r2d2::ConnectionManager, types::TimestampTzValue,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::{store::StoreError, types::User};

#[derive(Clone)]
pub struct UserStore(Pool<ConnectionManager>);

impl UserStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(
        &self,
        user: &User,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        connection.execute(
            r#"
            INSERT INTO "user" (
                id,
                source_language_id,
                username,
                password_hash,
                created_on,
                updated_on
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            Parameters::positional(&[
                &user.id,
                &user.source_language_id,
                &user.username,
                &user.password_hash,
                &user.created_on.to_rfc3339(),
                &user.updated_on.to_rfc3339()
            ]),
        )?;

        Ok(())
    }

    pub fn get_by_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<User>, StoreError> {
        let connection = self.0.get()?;

        let result = connection.query(
            r#"
            SELECT
                id,
                source_language_id,
                username,
                password_hash,
                created_on,
                updated_on
            FROM "user"
            WHERE id = $1
            "#,
            Parameters::positional(&[
                &user_id,
            ]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? == 0 {
                continue;
            }

            let id = chunk.get_vector_at::<Uuid>(0)?.get(0)?.unwrap();
            let source_language_id = chunk.get_vector_at::<String>(1)?.get(0)?.unwrap().into();
            let username = chunk.get_vector_at::<String>(2)?.get(0)?.unwrap().into();
            let password_hash = chunk.get_vector_at::<String>(3)?.get(0)?.unwrap().into();
            let created_on = chunk.get_vector_at::<TimestampTzValue>(4)?.get(0)?.unwrap();
            let created_on = Option::<chrono::DateTime<chrono::Utc>>::from(*created_on).unwrap();
            let updated_on = chunk.get_vector_at::<TimestampTzValue>(4)?.get(0)?.unwrap();
            let updated_on = Option::<chrono::DateTime<chrono::Utc>>::from(*updated_on).unwrap();

            let entity = User {
                id,
                source_language_id,
                username,
                password_hash,
                created_on,
                updated_on
            };

            return Ok(Some(entity));
        }

        Ok(None)
    }
}