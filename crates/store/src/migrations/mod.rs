use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::PooledConnection;
use uuid::Uuid;

use crate::queries::*;

pub const MIGRATIONS: &[(&str, &str)] = &[
    (include_str!("./001_init.sql"), "migrations/001_init.sql"),
    (include_str!("./002_data_languages.sql"), "migrations/002_data_languages.sql"),
    (include_str!("./003_data_script.sql"), "migrations/003_data_script.sql"),
    (include_str!("./004_data_lexeme.sql"), "migrations/004_data_lexeme.sql"),
    (include_str!("./005_data_character.sql"), "migrations/005_data_character.sql"),
];

pub fn apply_migrations(connection: PooledConnection<DuckdbConnectionManager>) -> Result<()> {
    let table_exists: Option<bool> = connection
        .query_row(CHECK_MIGRATIONS_TABLE, [], |row| row.get(0))
        .ok()
        .filter(|pr| *pr);

    for (sql, file) in MIGRATIONS.iter() {
        let applied: Option<Uuid> = if table_exists.is_some() {
            connection
                .query_row(GET_MIGRATION_BY_FILE_NAME, [file], |row| row.get(0))
                .optional()?
        } else {
            None
        };

        if applied.is_some() {
            continue;
        }

        connection.execute_batch(sql).with_context(|| format!("An error occurred whilst executing {file}"))?;

        let params = params![Uuid::now_v7(), *file, Utc::now()];
        connection.execute(
            INSERT_MIGRATION,
            params,
        )?;
    }
    
    let version: &str = env!("CARGO_PKG_VERSION");
    connection.execute(UPDATE_METADATA, params![version, Utc::now()])?;

    Ok(())
}