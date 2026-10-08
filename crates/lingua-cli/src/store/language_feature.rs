use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::{LanguageFeature, LanguageFeatureBadge};
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;

#[derive(Clone)]
pub struct LanguageFeatureStore(Pool<ConnectionManager>);

impl LanguageFeatureStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn get(&self, id: &str) -> Result<Option<LanguageFeature>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(&self) -> Result<Vec<LanguageFeature>, StoreError> {
        let connection = self.0.get()?;

        read_many(&connection, queries::LIST, Parameters::None)
    }

    pub fn for_language(&self, language_id: &str) -> Result<Vec<LanguageFeatureBadge>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::FOR_LANGUAGE,
            Parameters::positional(&[&language_id]),
        )
    }
}

mod queries {
    pub(super) const GET: &str = r#"
        SELECT
            id,
            name,
            description,
            group_code,
            display_order
        FROM language_feature
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            name,
            description,
            group_code,
            display_order
        FROM language_feature
        ORDER BY
            display_order,
            id
    "#;

    pub(super) const FOR_LANGUAGE: &str = r#"
        SELECT
            assignment.language_id,
            feature.id AS feature_id,
            feature.name AS feature_name,
            feature.description AS feature_description,
            feature.group_code,
            feature.display_order,
            option.code AS value_code,
            option.badge_label,
            option.description AS value_description,
            assignment.notes,
            assignment.example
        FROM language_feature_assignment assignment
        JOIN language_feature feature
            ON feature.id = assignment.feature_id
        JOIN language_feature_value option
            ON option.feature_id = assignment.feature_id
            AND option.code = assignment.value_code
        WHERE
            assignment.language_id = $1
        ORDER BY
            feature.display_order,
            feature.id
    "#;
}
