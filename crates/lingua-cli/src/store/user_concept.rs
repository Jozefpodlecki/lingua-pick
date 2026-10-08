use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::UserConcept;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserConceptStore(Pool<ConnectionManager>);

impl UserConceptStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn refresh_from_evidence(
        &self,
        user_id: Uuid,
        target_language_id: &str,
    ) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        refresh_from_evidence(&connection, user_id, target_language_id)
    }

    pub fn insert(&self, model: &UserConcept) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        let last_seen = model.last_seen.map(|value| value.to_rfc3339());
        let updated_on = model.updated_on.to_rfc3339();
        connection.execute(
            queries::INSERT,
            Parameters::positional(&[
                &model.user_id,
                &model.target_language_id,
                &model.concept_id,
                &model.attempts,
                &model.correct,
                &model.mastery,
                &model.confidence,
                &last_seen,
                &updated_on,
            ]),
        )?;
        Ok(())
    }

    pub fn get(
        &self,
        user_id: Uuid,
        target_language_id: &str,
        concept_id: Uuid,
    ) -> Result<Option<UserConcept>, StoreError> {
        let connection = self.0.get()?;

        read_one(
            &connection,
            queries::GET,
            Parameters::positional(&[&user_id, &target_language_id, &concept_id]),
        )
    }

    pub fn list(
        &self,
        user_id: Uuid,
        target_language_id: &str,
    ) -> Result<Vec<UserConcept>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&user_id, &target_language_id]),
        )
    }
}

pub(super) fn refresh_from_evidence(
    connection: &duckdb_neo::connection::Connection,
    user_id: Uuid,
    target_language_id: &str,
) -> Result<(), StoreError> {
    connection.execute(
        queries::REFRESH_FROM_EVIDENCE,
        Parameters::positional(&[&user_id, &target_language_id]),
    )?;
    Ok(())
}

mod queries {
    pub(super) const INSERT: &str = r#"
        INSERT INTO user_concept
        (
            user_id,
            target_language_id,
            concept_id,
            attempts,
            correct,
            mastery,
            confidence,
            last_seen,
            updated_on
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9
        )
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            user_id,
            target_language_id,
            concept_id,
            attempts,
            correct,
            mastery,
            confidence,
            strftime(last_seen AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS last_seen,
            strftime(updated_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS updated_on
        FROM user_concept
        WHERE
            user_id = $1
            AND target_language_id = $2
            AND concept_id = $3
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            user_id,
            target_language_id,
            concept_id,
            attempts,
            correct,
            mastery,
            confidence,
            strftime(last_seen AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS last_seen,
            strftime(updated_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS updated_on
        FROM user_concept
        WHERE
            user_id = $1
            AND target_language_id = $2
        ORDER BY
            concept_id
    "#;

    pub(super) const REFRESH_FROM_EVIDENCE: &str = r#"
        INSERT INTO user_concept
        (
            user_id,
            target_language_id,
            concept_id,
            attempts,
            correct,
            last_seen
        )
        SELECT
            user_id,
            target_language_id,
            concept_id,
            count(*) FILTER (WHERE correct IS NOT NULL AND NOT assisted),
            count(*) FILTER (WHERE correct = TRUE AND NOT assisted),
            max(occurred_on)
        FROM learning_evidence
        WHERE
            user_id = $1
            AND target_language_id = $2
        GROUP BY
            user_id,
            target_language_id,
            concept_id
        ON CONFLICT (user_id, target_language_id, concept_id)
        DO UPDATE
        SET
            attempts = excluded.attempts,
            correct = excluded.correct,
            last_seen = excluded.last_seen,
            updated_on = now()
    "#;
}
