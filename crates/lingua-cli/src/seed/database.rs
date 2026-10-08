use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use duckdb_neo::{
    environment::{Environment, StorageLocation},
    r2d2::ConnectionManager,
};

use super::{DatabaseMode, SeedError};
use crate::migration::{apply_migrations, verify_migrations};

pub(super) struct Database {
    pub pool: r2d2::Pool<ConnectionManager>,
    pub is_new: bool,
}

pub(super) fn open(path: &Path, mode: DatabaseMode) -> Result<Database, SeedError> {
    let environment = Environment::new()?;
    let (location, is_new) = match mode {
        DatabaseMode::InMemory => (StorageLocation::InMemory, true),
        _ => prepare_file(&environment, path, mode)?,
    };
    let label = String::from(&location);
    let database = environment.open(location)?;
    let pool = r2d2::Pool::builder()
        .max_size(2)
        .build(ConnectionManager::new(&database))?;

    {
        let connection = pool.get()?;

        if is_new {
            let applied = apply_migrations(&connection)?;
            tracing::info!(
                database = %label,
                database_mode = %mode,
                migrations = applied,
                "Fresh database initialized"
            );
        } else {
            verify_migrations(&connection)?;
            tracing::info!(database = %label, "Existing database migrations verified");
        }
    }

    Ok(Database { pool, is_new })
}

fn prepare_file(
    environment: &Environment,
    path: &Path,
    mode: DatabaseMode,
) -> Result<(StorageLocation, bool), SeedError> {
    if path.as_os_str().is_empty() || path.to_str() == Some(":memory:") {
        return Err(SeedError::InvalidState(
            "file database modes require a persistent file path".into(),
        ));
    }

    let path_text = path
        .to_str()
        .ok_or_else(|| SeedError::InvalidState("database path is not valid UTF-8".into()))?;
    let exists = existing_file(path)?;

    if mode == DatabaseMode::ExistingOnly && !exists {
        return Err(SeedError::InvalidState(format!(
            "existing-only database mode requires an existing file: {}",
            path.display()
        )));
    }

    if mode == DatabaseMode::Recreate {
        recreate(environment, path, path_text, exists)?;
    }

    let is_new = !exists || mode == DatabaseMode::Recreate;
    if is_new {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }
    }

    Ok((StorageLocation::OnDisk(path_text.into()), is_new))
}

fn existing_file(path: &Path) -> Result<bool, SeedError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() => Ok(true),
        Ok(_) => Err(SeedError::InvalidState(format!(
            "database and WAL paths must be regular files, not directories or links: {}",
            path.display()
        ))),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

fn recreate(
    environment: &Environment,
    path: &Path,
    path_text: &str,
    exists: bool,
) -> Result<(), SeedError> {
    let mut wal_name = path.as_os_str().to_os_string();
    wal_name.push(".wal");
    let wal_path = PathBuf::from(wal_name);
    existing_file(&wal_path)?;

    if exists {
        let database = environment.open(StorageLocation::OnDisk(path_text.into()))?;
        drop(database);
    }

    tracing::warn!(database = %path.display(), "Recreating disposable database");
    remove_file_if_present(path)?;
    remove_file_if_present(&wal_path)?;

    Ok(())
}

fn remove_file_if_present(path: &Path) -> Result<(), SeedError> {
    match std::fs::remove_file(path) {
        Ok(()) => {
            tracing::info!(file = %path.display(), "Removed disposable database file");
            Ok(())
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
