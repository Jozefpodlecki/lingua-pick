use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::LanguageFeatureAssignment;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct LanguageFeatureAssignmentStore(Pool<ConnectionManager>);

impl LanguageFeatureAssignmentStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get(
        &self,
        language_id: &str,
        feature_id: &str,
    ) -> Result<Option<LanguageFeatureAssignment>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&language_id, &feature_id]),
        )
    }

    pub fn list(&self, language_id: &str) -> Result<Vec<LanguageFeatureAssignment>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&language_id]),
        )
    }
}

mod queries {
    pub(super) const GET: &str = r#"
        SELECT
            language_id,
            feature_id,
            value_code,
            notes,
            example
        FROM language_feature_assignment
        WHERE
            language_id = $1
            AND feature_id = $2
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            language_id,
            feature_id,
            value_code,
            notes,
            example
        FROM language_feature_assignment
        WHERE
            language_id = $1
        ORDER BY
            feature_id
    "#;
}
