use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use duckdb::{OptionalExt, Row, params};
use uuid::Uuid;

use crate::{
    ContentSource, ExerciseResult, LinguaStore, NewSession, NewSessionExercise, Result,
    SessionExerciseRecord, SessionId, SessionRecord, SessionStatus, StoreError,
};

const SESSION_COLUMNS: &str = r#"
id,
target_language_id,
content_source,
status,
started_at,
updated_at,
completed_at,
total_exercises,
answered_exercises,
correct_exercises
"#;

pub struct SessionRepository<'a> {
    store: &'a LinguaStore,
}

impl<'a> SessionRepository<'a> {
    pub(crate) const fn new(store: &'a LinguaStore) -> Self {
        Self { store }
    }

    pub fn create(
        &self,
        session: &NewSession,
        exercises: &[NewSessionExercise],
    ) -> Result<SessionRecord> {
        validate_new_session(session, exercises)?;

        let serialized = exercises
            .iter()
            .map(|exercise| serde_json::to_string(&exercise.payload))
            .collect::<core::result::Result<Vec<_>, _>>()?;
        let mut connection = self.store.connection()?;
        let transaction = connection.transaction()?;
        transaction.execute(
            r#"
            INSERT INTO learning_session (
                id,
                target_language_id,
                content_source,
                total_exercises
            ) VALUES (?, ?, ?, ?)
            "#,
            params![
                session.id.0,
                session.target_language_id,
                session.content_source.as_str(),
                session.total_exercises,
            ],
        )?;

        for (exercise, payload) in exercises.iter().zip(serialized) {
            transaction.execute(
                r#"
                INSERT INTO session_exercise (
                    session_id,
                    position,
                    exercise_id,
                    exercise_kind,
                    payload_json
                ) VALUES (?, ?, ?, ?, ?)
                "#,
                params![
                    session.id.0,
                    exercise.position,
                    exercise.exercise_id,
                    exercise.exercise_kind,
                    payload,
                ],
            )?;
        }

        transaction.commit()?;
        drop(connection);

        self.get(session.id)?.ok_or(StoreError::NotFound)
    }

    pub fn record_result(&self, session_id: SessionId, result: &ExerciseResult) -> Result<()> {
        if result.position == 0 {
            return Err(StoreError::InvalidRecord(String::from(
                "Exercise position must start at one.",
            )));
        }

        let answer = serde_json::to_string(&result.answer)?;
        let mut connection = self.store.connection()?;
        let transaction = connection.transaction()?;
        let updated = transaction.execute(
            r#"
            UPDATE session_exercise
            SET answer_json = ?, correct = ?, answered_at = current_timestamp
            WHERE session_id = ? AND position = ?
            "#,
            params![answer, result.correct, session_id.0, result.position],
        )?;

        if updated == 0 {
            return Err(StoreError::NotFound);
        }

        transaction.execute(
            r#"
            UPDATE learning_session
            SET
                answered_exercises = (
                    SELECT count(*)
                    FROM session_exercise
                    WHERE session_id = ? AND answered_at IS NOT NULL
                ),
                correct_exercises = (
                    SELECT count(*)
                    FROM session_exercise
                    WHERE session_id = ? AND correct IS TRUE
                ),
                updated_at = current_timestamp
            WHERE id = ?
            "#,
            params![session_id.0, session_id.0, session_id.0],
        )?;
        transaction.commit()?;

        Ok(())
    }

    pub fn complete(&self, session_id: SessionId) -> Result<SessionRecord> {
        let connection = self.store.connection()?;
        let updated = connection.execute(
            r#"
            UPDATE learning_session
            SET
                status = 'completed',
                completed_at = current_timestamp,
                updated_at = current_timestamp
            WHERE id = ? AND answered_exercises = total_exercises
            "#,
            [session_id.0],
        )?;
        drop(connection);

        if updated == 0 {
            return match self.get(session_id)? {
                Some(_) => Err(StoreError::IncompleteSession),
                None => Err(StoreError::NotFound),
            };
        }

        self.get(session_id)?.ok_or(StoreError::NotFound)
    }

    pub fn get(&self, session_id: SessionId) -> Result<Option<SessionRecord>> {
        let connection = self.store.connection()?;
        let sql = format!("SELECT {SESSION_COLUMNS} FROM learning_session WHERE id = ?");
        let raw = connection
            .query_row(&sql, [session_id.0], read_session)
            .optional()?;

        raw.map(parse_session).transpose()
    }

    pub fn recent_for_target(
        &self,
        target_language_id: &str,
        limit: u32,
    ) -> Result<Vec<SessionRecord>> {
        if target_language_id.trim().is_empty() {
            return Err(StoreError::InvalidRecord(String::from(
                "Target language identifier must not be blank.",
            )));
        }

        if limit == 0 || limit > 500 {
            return Err(StoreError::InvalidRecord(String::from(
                "History limit must be between 1 and 500.",
            )));
        }

        let connection = self.store.connection()?;
        let sql = format!(
            "SELECT {SESSION_COLUMNS} FROM learning_session WHERE target_language_id = ? ORDER BY started_at DESC, id DESC LIMIT ?"
        );
        let mut statement = connection.prepare(&sql)?;
        let rows = statement.query_map(params![target_language_id, limit], read_session)?;
        let mut sessions = Vec::new();

        for row in rows {
            sessions.push(parse_session(row?)?);
        }

        Ok(sessions)
    }

    pub fn exercises(&self, session_id: SessionId) -> Result<Vec<SessionExerciseRecord>> {
        let connection = self.store.connection()?;
        let mut statement = connection.prepare(
            r#"
            SELECT
                session_id,
                position,
                exercise_id,
                exercise_kind,
                payload_json,
                answer_json,
                correct,
                presented_at,
                answered_at
            FROM session_exercise
            WHERE session_id = ?
            ORDER BY position
            "#,
        )?;
        let rows = statement.query_map([session_id.0], |row| {
            Ok(RawExercise {
                session_id: row.get(0)?,
                position: row.get(1)?,
                exercise_id: row.get(2)?,
                exercise_kind: row.get(3)?,
                payload_json: row.get(4)?,
                answer_json: row.get(5)?,
                correct: row.get(6)?,
                presented_at: row.get(7)?,
                answered_at: row.get(8)?,
            })
        })?;
        let mut exercises = Vec::new();

        for row in rows {
            let raw = row?;
            exercises.push(SessionExerciseRecord {
                session_id: SessionId(raw.session_id),
                position: raw.position,
                exercise_id: raw.exercise_id,
                exercise_kind: raw.exercise_kind,
                payload: serde_json::from_str(&raw.payload_json)?,
                answer: raw
                    .answer_json
                    .map(|answer| serde_json::from_str(&answer))
                    .transpose()?,
                correct: raw.correct,
                presented_at: raw.presented_at,
                answered_at: raw.answered_at,
            });
        }

        Ok(exercises)
    }
}

type RawSession = (
    Uuid,
    String,
    String,
    String,
    DateTime<Utc>,
    DateTime<Utc>,
    Option<DateTime<Utc>>,
    u16,
    u16,
    u16,
);

struct RawExercise {
    session_id: Uuid,
    position: u16,
    exercise_id: String,
    exercise_kind: String,
    payload_json: String,
    answer_json: Option<String>,
    correct: Option<bool>,
    presented_at: DateTime<Utc>,
    answered_at: Option<DateTime<Utc>>,
}

fn read_session(row: &Row<'_>) -> duckdb::Result<RawSession> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
    ))
}

fn parse_session(raw: RawSession) -> Result<SessionRecord> {
    let content_source = ContentSource::parse(&raw.2)
        .ok_or_else(|| StoreError::InvalidRecord(format!("Unknown content source {}.", raw.2)))?;
    let status = SessionStatus::parse(&raw.3)
        .ok_or_else(|| StoreError::InvalidRecord(format!("Unknown session status {}.", raw.3)))?;

    Ok(SessionRecord {
        id: SessionId(raw.0),
        target_language_id: raw.1,
        content_source,
        status,
        started_at: raw.4,
        updated_at: raw.5,
        completed_at: raw.6,
        total_exercises: raw.7,
        answered_exercises: raw.8,
        correct_exercises: raw.9,
    })
}

fn validate_new_session(session: &NewSession, exercises: &[NewSessionExercise]) -> Result<()> {
    if session.target_language_id.trim().is_empty() {
        return Err(StoreError::InvalidRecord(String::from(
            "Target language identifier must not be blank.",
        )));
    }

    if session.total_exercises == 0 || usize::from(session.total_exercises) != exercises.len() {
        return Err(StoreError::InvalidRecord(String::from(
            "Exercise count must match the nonzero session total.",
        )));
    }

    let mut exercise_ids = BTreeSet::new();

    for (index, exercise) in exercises.iter().enumerate() {
        if usize::from(exercise.position) != index + 1 {
            return Err(StoreError::InvalidRecord(String::from(
                "Exercise positions must be contiguous and start at one.",
            )));
        }

        if exercise.exercise_id.trim().is_empty() || exercise.exercise_kind.trim().is_empty() {
            return Err(StoreError::InvalidRecord(String::from(
                "Exercise identifier and kind must not be blank.",
            )));
        }

        if !exercise_ids.insert(exercise.exercise_id.as_str()) {
            return Err(StoreError::InvalidRecord(String::from(
                "Exercise identifiers must be unique within a session.",
            )));
        }
    }

    Ok(())
}
