use chrono::Utc;
use duckdb_neo::{Parameters, r2d2::ConnectionManager};
use uuid::Uuid;

use super::SeedError;
use crate::{
    store::{LanguageStore, SessionStore, UserStatsStore, UserStore},
    utils::hash_password,
};

pub(super) struct State {
    pub user_id: Uuid,
    pub session_id: Uuid,
}

fn validate_languages(
    pool: &r2d2::Pool<ConnectionManager>,
    source: &str,
    target: &str,
) -> Result<(), SeedError> {
    let languages = LanguageStore::new(pool.clone());
    for id in [source, target] {
        if languages.get_by_id(id)?.is_none() {
            return Err(SeedError::LanguageNotFound(id.into()));
        }
    }
    Ok(())
}

pub(super) fn create(
    pool: &r2d2::Pool<ConnectionManager>,
    name: &str,
    source: &str,
    target: &str,
) -> Result<State, SeedError> {
    validate_languages(pool, source, target)?;
    let state = State {
        user_id: Uuid::now_v7(),
        session_id: Uuid::now_v7(),
    };
    let now = Utc::now().to_rfc3339();
    let password = hash_password("123")?;
    let connection = pool.get()?;
    connection.execute("BEGIN TRANSACTION", Parameters::None)?;
    let result = (|| -> Result<(), SeedError> {
        connection.execute(
            "INSERT INTO user (id, source_language_id, username, password_hash, created_on, updated_on) VALUES ($1, $2, $3, $4, $5, $5)",
            Parameters::positional(&[&state.user_id, &source, &name, &password, &now]),
        )?;
        connection.execute(
            "INSERT INTO user_stats (user_id, target_language_id, created_on, updated_on, struggling_categories) VALUES ($1, $2, $3, $3, '[]')",
            Parameters::positional(&[&state.user_id, &target, &now]),
        )?;
        connection.execute(
            "INSERT INTO session (id, user_id, target_language_id, created_on, exercise_count, max_exercise_count, last_exercise_id) VALUES ($1, $2, $3, $4, 0, 10, NULL)",
            Parameters::positional(&[&state.session_id, &state.user_id, &target, &now]),
        )?;
        Ok(())
    })();
    match result {
        Ok(()) => {
            if let Err(error) = connection.execute("COMMIT", Parameters::None) {
                let _ = connection.execute("ROLLBACK", Parameters::None);
                return Err(error.into());
            }
        }
        Err(error) => {
            let _ = connection.execute("ROLLBACK", Parameters::None);
            return Err(error);
        }
    }
    Ok(state)
}

pub(super) fn resolve(
    pool: &r2d2::Pool<ConnectionManager>,
    name: &str,
    source: &str,
    target: &str,
) -> Result<State, SeedError> {
    validate_languages(pool, source, target)?;
    let users: Vec<_> = UserStore::new(pool.clone())
        .list()?
        .into_iter()
        .filter(|u| u.username == name)
        .collect();
    if users.len() != 1 {
        return Err(SeedError::InvalidState(
            "existing database must contain exactly one requested learner; it will not be reseeded"
                .into(),
        ));
    }
    let user = &users[0];
    if user.source_language_id != source {
        return Err(SeedError::InvalidState(
            "saved learner source language differs from the requested source".into(),
        ));
    }
    if UserStatsStore::new(pool.clone())
        .get(user.id, target)?
        .is_none()
    {
        return Err(SeedError::InvalidState(
            "requested target has no saved learner statistics".into(),
        ));
    }
    let sessions = SessionStore::new(pool.clone()).list(user.id, target)?;
    let session = sessions
        .iter()
        .rev()
        .find(|s| s.exercise_count < s.max_exercise_count)
        .or_else(|| sessions.last())
        .ok_or_else(|| SeedError::InvalidState("requested target has no saved session".into()))?;
    Ok(State {
        user_id: user.id,
        session_id: session.id,
    })
}
