use std::collections::BTreeMap;

use duckdb_neo::{Connection, params};
use sha2::{Digest, Sha256};

use crate::{Result, StoreError};

const MIGRATION_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS schema_migration (
    version UINTEGER PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    checksum VARCHAR NOT NULL,
    applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp
);
"#;

const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "initial_learning_history",
    sql: include_str!("001_initial_learning_history.sql"),
}];

struct Migration {
    version: u32,
    name: &'static str,
    sql: &'static str,
}

pub fn apply_migrations(connection: &mut Connection) -> Result<usize> {
    connection.execute_batch(MIGRATION_TABLE)?;

    let applied = applied_migrations(connection)?;
    validate_applied(&applied)?;

    let mut applied_count = 0;

    for migration in MIGRATIONS {
        if applied.contains_key(&migration.version) {
            continue;
        }

        let checksum = checksum(migration.sql);
        let transaction = connection.transaction()?;
        transaction.execute_batch(migration.sql)?;
        transaction.execute(
            "INSERT INTO schema_migration (version, name, checksum) VALUES (?, ?, ?)",
            params![migration.version, migration.name, checksum],
        )?;
        transaction.commit()?;
        applied_count += 1;
    }

    Ok(applied_count)
}

fn applied_migrations(connection: &Connection) -> Result<BTreeMap<u32, (String, String)>> {
    let mut statement = connection
        .prepare("SELECT version, name, checksum FROM schema_migration ORDER BY version")?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, u32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    let mut applied = BTreeMap::new();

    for row in rows {
        let (version, name, checksum) = row?;
        applied.insert(version, (name, checksum));
    }

    Ok(applied)
}

fn validate_applied(applied: &BTreeMap<u32, (String, String)>) -> Result<()> {
    for (version, (recorded_name, recorded_checksum)) in applied {
        let Some(migration) = MIGRATIONS
            .iter()
            .find(|migration| migration.version == *version)
        else {
            return Err(StoreError::UnknownMigration(*version));
        };
        let expected_checksum = checksum(migration.sql);

        if recorded_name != migration.name || recorded_checksum != &expected_checksum {
            return Err(StoreError::ChangedMigration { version: *version });
        }
    }

    Ok(())
}

fn checksum(sql: &str) -> String {
    format!("{:x}", Sha256::digest(sql.as_bytes()))
}
