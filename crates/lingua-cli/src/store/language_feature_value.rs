use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::LanguageFeatureValue;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct LanguageFeatureValueStore(Pool<ConnectionManager>);

impl LanguageFeatureValueStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get(
        &self,
        feature_id: &str,
        code: &str,
    ) -> Result<Option<LanguageFeatureValue>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&feature_id, &code]),
        )
    }

    pub fn list(&self, feature_id: &str) -> Result<Vec<LanguageFeatureValue>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&feature_id]),
        )
    }
}

mod queries {
    pub(super) const GET: &str = r#"
        SELECT
            feature_id,
            code,
            badge_label,
            description
        FROM language_feature_value
        WHERE
            feature_id = $1
            AND code = $2
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            feature_id,
            code,
            badge_label,
            description
        FROM language_feature_value
        WHERE
            feature_id = $1
        ORDER BY
            code
    "#;
}
