use std::collections::BTreeMap;
use thiserror::Error;
use duckdb_neo::{Parameters, connection::Connection, query_result::StatementType::Transaction};
use sha2::{Digest, Sha256};

const MIGRATION_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS schema_migration (
    version UINTEGER PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    checksum VARCHAR NOT NULL,
    applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp
);
"#;

struct Migration {
    version: u32,
    name: &'static str,
    sql: &'static str,
}

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("could not create migrations table: {0}")]
    CouldNotCreateMigrationsTable(duckdb_neo::error::Error),

    #[error("database error: {0}")]
    Database(#[from] duckdb_neo::error::Error),

    #[error("unknown migration version: {0}")]
    UnknownMigration(u32),

    #[error("migration {version} has changed after being applied")]
    ChangedMigration {
        version: u32,
    },
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial",
        sql: include_str!("migration/001_initial.sql"),
    },
    Migration {
        version: 2,
        name: "data_category",
        sql: include_str!("migration/002_data_category.sql"),
    },
    Migration {
        version: 3,
        name: "data_exercise_definition",
        sql: include_str!("migration/003_data_exercise_definition.sql"),
    }
];

#[derive(Debug, Clone)]
struct AppliedMigration {
    name: String,
    checksum: String,
}

#[derive(Debug, Default)]
struct AppliedMigrations {
    migrations: BTreeMap<u32, AppliedMigration>,
}

impl AppliedMigrations {
    fn contains(&self, version: u32) -> bool {
        self.migrations.contains_key(&version)
    }

    fn insert(
        &mut self,
        version: u32,
        name: String,
        checksum: String,
    ) {
        self.migrations.insert(
            version,
            AppliedMigration {
                name,
                checksum,
            },
        );
    }

    fn iter(
        &self,
    ) -> impl Iterator<Item = (&u32, &AppliedMigration)> {
        self.migrations.iter()
    }
}

pub fn apply_migrations(
    connection: &Connection,
) -> Result<usize, MigrationError> {
    connection
        .execute(MIGRATION_TABLE, Parameters::None)
        .map_err(MigrationError::CouldNotCreateMigrationsTable)?;

    let applied = applied_migrations(connection)?;

    validate_applied(&applied)?;

    let mut applied_count = 0;

    for migration in MIGRATIONS {
        if applied.contains(migration.version) {
            continue;
        }

        apply_migration(connection, migration)?;

        applied_count += 1;
    }

    Ok(applied_count)
}

fn apply_migration(
    connection: &Connection,
    migration: &Migration,
) -> Result<(), MigrationError> {
    connection.execute(
        "BEGIN TRANSACTION",
        Parameters::None,
    )?;

    let result = (|| {
        execute_batch(connection, migration.sql)?;

        let checksum = checksum(migration.sql);

        connection.execute(
            r#"
            INSERT INTO schema_migration (
                version,
                name,
                checksum
            )
            VALUES ($1, $2, $3)
            "#,
            Parameters::positional(&[
                &migration.version,
                &migration.name,
                &checksum,
            ]),
        )?;

        Ok::<_, MigrationError>(())
    })();

    match result {
        Ok(()) => {
            connection.execute(
                "COMMIT",
                Parameters::None,
            )?;

            Ok(())
        }

        Err(error) => {
            let _ = connection.execute(
                "ROLLBACK",
                Parameters::None,
            );

            Err(error)
        }
    }
}

fn execute_batch(
    connection: &Connection,
    sql: &str,
) -> Result<(), MigrationError> {
    let statements = connection.parse(sql)?;

    for statement in statements {
        let statement = statement?;

        connection.execute(
            statement,
            Parameters::None,
        )?;
    }

    Ok(())
}

fn applied_migrations(
    connection: &Connection,
) -> Result<AppliedMigrations, MigrationError> {
    let result = connection.query(
        r#"
        SELECT version, name, checksum
        FROM schema_migration
        ORDER BY version
        "#,
        Parameters::None,
    )?;

    let mut applied = AppliedMigrations::default();

    for chunk in result {
        let chunk = chunk?;

        let versions = chunk.get_vector_at::<u32>(0)?;
        let names = chunk.get_vector_at::<String>(1)?;
        let checksums = chunk.get_vector_at::<String>(2)?;

        for row in 0..chunk.row_count()? {
            let version = versions
                .get(row)?
                .copied()
                .expect("version is NOT NULL");

            let name = names
                .get(row)?
                .map(String::from)
                .expect("name is NOT NULL");

            let checksum = checksums
                .get(row)?
                .map(String::from)
                .expect("checksum is NOT NULL");

            applied.insert(version, name, checksum);
        }
    }

    Ok(applied)
}

fn validate_applied(
    applied: &AppliedMigrations,
) -> Result<(), MigrationError> {
    for (version, applied) in applied.iter() {
        let Some(migration) = MIGRATIONS
            .iter()
            .find(|migration| migration.version == *version)
        else {
            return Err(
                MigrationError::UnknownMigration(*version),
            );
        };

        let expected_checksum = checksum(migration.sql);

        if applied.name != migration.name
            || applied.checksum != expected_checksum
        {
            return Err(
                MigrationError::ChangedMigration {
                    version: *version,
                },
            );
        }
    }

    Ok(())
}

fn checksum(sql: &str) -> String {
    hex::encode(Sha256::digest(sql.as_bytes()))
}