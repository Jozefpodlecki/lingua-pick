use chrono::{DateTime, Utc};
use duckdb_neo::{
    Parameters, r2d2::ConnectionManager, types::TimestampTzValue,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::{store::StoreError, types::UserStats};

#[derive(Clone)]
pub struct UserStatsStore(Pool<ConnectionManager>);

impl UserStatsStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn insert(
        &self,
        model: &UserStats,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let struggling_categories =
            serde_json::to_string(&model.struggling_categories)?;

        connection.execute(
            r#"
            INSERT INTO user_stats (
                user_id,
                target_language_id,
                created_on,
                updated_on,
                struggling_categories
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            Parameters::positional(&[
                &model.user_id,
                &model.target_language_id,
                &model.created_on.to_rfc3339(),
                &model.updated_on.to_rfc3339(),
                &struggling_categories
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

        let result = connection.query(
            r#"
            SELECT
                user_id,
                target_language_id,
                created_on,
                updated_on,
                last_exercise::VARCHAR,
                struggling_categories::VARCHAR
            FROM user_stats
            WHERE user_id = $1
              AND target_language_id = $2
            "#,
            Parameters::positional(&[
                &user_id,
                &target_language_id,
            ]),
        )?;

        for chunk in result {
            let chunk = chunk?;

            if chunk.row_count()? == 0 {
                continue;
            }

            let user_id = chunk.get_vector_at::<Uuid>(0)?.get(0)?.unwrap();
            let target_language_id = chunk.get_vector_at::<String>(1)?.get(0)?.unwrap().into();
            let created_on = chunk.get_vector_at::<TimestampTzValue>(2)?.get(0)?.unwrap();
            let created_on = Option::<DateTime<Utc>>::from(*created_on).unwrap();
            let updated_on = chunk.get_vector_at::<TimestampTzValue>(3)?.get(0)?.unwrap();
            let updated_on = Option::<DateTime<Utc>>::from(*updated_on).unwrap();
            let struggling_categories = chunk.get_vector_at::<String>(5)?.get(0)?.unwrap();
            let struggling_categories = serde_json::from_str(struggling_categories)?;

            return Ok(Some(UserStats {
                user_id,
                target_language_id,
                created_on,
                updated_on,
                struggling_categories
            }));
        }

        Ok(None)
    }

    pub fn update(
        &self,
        model: &UserStats,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let struggling_categories =
            serde_json::to_string(&model.struggling_categories)?;

        connection.execute(
            r#"
            UPDATE user_stats
            SET
                updated_on = $1,
                struggling_categories = $2,
            WHERE user_id = $3
              AND target_language_id = $4
            "#,
            Parameters::positional(&[
                &model.updated_on.to_rfc3339(),
                &struggling_categories,
                &model.user_id,
                &model.target_language_id,
            ]),
        )?;

        Ok(())
    }
}