use super::{
    StoreError,
    query::{read_many, read_one},
};
use crate::types::{ConceptEvidence, LearningEvidence};
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct LearningEvidenceStore(Pool<ConnectionManager>);

impl LearningEvidenceStore {
    pub const fn new(pool: Pool<ConnectionManager>) -> Self {
        Self(pool)
    }

    pub fn summarize_concept(
        &self,
        user_id: Uuid,
        target_language_id: &str,
        concept_id: Uuid,
    ) -> Result<Vec<ConceptEvidence>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::SUMMARIZE_CONCEPT,
            Parameters::positional(&[&user_id, &target_language_id, &concept_id]),
        )
    }

    pub fn insert(&self, model: &LearningEvidence) -> Result<(), StoreError> {
        let connection = self.0.get()?;

        insert_evidence(&connection, model)
    }

    pub fn get(&self, id: Uuid) -> Result<Option<LearningEvidence>, StoreError> {
        let connection = self.0.get()?;

        read_one(&connection, queries::GET, Parameters::positional(&[&id]))
    }

    pub fn list(
        &self,
        user_id: Uuid,
        target_language_id: &str,
    ) -> Result<Vec<LearningEvidence>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::LIST,
            Parameters::positional(&[&user_id, &target_language_id]),
        )
    }

    pub fn summarize(
        &self,
        user_id: Uuid,
        target_language_id: &str,
    ) -> Result<Vec<ConceptEvidence>, StoreError> {
        let connection = self.0.get()?;

        read_many(
            &connection,
            queries::SUMMARIZE,
            Parameters::positional(&[&user_id, &target_language_id]),
        )
    }
}

pub(super) fn insert_evidence(
    connection: &duckdb_neo::connection::Connection,
    model: &LearningEvidence,
) -> Result<(), StoreError> {
    if model.event_key.trim().is_empty()
        || model.evaluator.trim().is_empty()
        || model.policy_version.trim().is_empty()
    {
        return Err(StoreError::InvalidInput(
            "evidence requires event, evaluator and policy identities",
        ));
    }
    let evidence_mode = model.evidence_mode.as_str();
    let occurred_on = model.occurred_on.to_rfc3339();
    let detail = serde_json::to_string(&model.detail)?;
    let changed = connection.execute(
        queries::INSERT_EVIDENCE,
        Parameters::positional(&[
            &model.id,
            &model.user_id,
            &model.target_language_id,
            &model.exercise_id,
            &model.concept_id,
            &model.event_key,
            &evidence_mode,
            &model.correct,
            &model.assisted,
            &occurred_on,
            &model.evaluator,
            &model.policy_version,
            &detail,
        ]),
    )?;
    if changed != 1 {
        return Err(StoreError::InvalidInput(
            "evidence must belong to the exercise learner, target and concepts",
        ));
    }

    Ok(())
}

mod queries {
    pub(super) const SUMMARIZE_CONCEPT: &str = r#"
        SELECT
            user_id,
            target_language_id,
            concept_id,
            evidence_mode,
            encounters,
            assessed_attempts,
            correct_attempts,
            assessed_sessions,
            strftime(last_seen AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS last_seen,
            accuracy
        FROM learner_concept_evidence
        WHERE
            user_id = $1
            AND target_language_id = $2
            AND concept_id = $3
        ORDER BY
            evidence_mode
    "#;

    pub(super) const GET: &str = r#"
        SELECT
            id,
            user_id,
            target_language_id,
            exercise_id,
            concept_id,
            event_key,
            evidence_mode,
            correct,
            assisted,
            strftime(occurred_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS occurred_on,
            evaluator,
            policy_version,
            detail
        FROM learning_evidence
        WHERE
            id = $1
    "#;

    pub(super) const LIST: &str = r#"
        SELECT
            id,
            user_id,
            target_language_id,
            exercise_id,
            concept_id,
            event_key,
            evidence_mode,
            correct,
            assisted,
            strftime(occurred_on AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS occurred_on,
            evaluator,
            policy_version,
            detail
        FROM learning_evidence
        WHERE
            user_id = $1
            AND target_language_id = $2
        ORDER BY
            occurred_on,
            id
    "#;

    pub(super) const SUMMARIZE: &str = r#"
        SELECT
            user_id,
            target_language_id,
            concept_id,
            evidence_mode,
            encounters,
            assessed_attempts,
            correct_attempts,
            assessed_sessions,
            strftime(last_seen AT TIME ZONE 'UTC', '%Y-%m-%dT%H:%M:%S.%fZ') AS last_seen,
            accuracy
        FROM learner_concept_evidence
        WHERE
            user_id = $1
            AND target_language_id = $2
        ORDER BY
            concept_id,
            evidence_mode
    "#;

    pub(super) const INSERT_EVIDENCE: &str = r#"
        INSERT INTO learning_evidence
        (
            id,
            user_id,
            target_language_id,
            exercise_id,
            concept_id,
            event_key,
            evidence_mode,
            correct,
            assisted,
            occurred_on,
            evaluator,
            policy_version,
            detail
        )
        SELECT
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11,
            $12,
            $13
        FROM exercise x
        JOIN session s
            ON s.id = x.session_id
        JOIN exercise_concept ec
            ON ec.exercise_id = x.id
        WHERE
            x.id = $4
            AND s.user_id = $2
            AND s.target_language_id = $3
            AND ec.concept_id = $5
    "#;
}
