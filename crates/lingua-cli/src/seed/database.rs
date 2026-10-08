use std::path::Path;

use duckdb_neo::{
    environment::{Environment, StorageLocation},
    r2d2::ConnectionManager,
};

use super::SeedError;
use crate::migration::{apply_migrations, verify_migrations};

pub(super) struct Database {
    pub pool: r2d2::Pool<ConnectionManager>,
    pub is_new: bool,
}

pub(super) fn open(path: &Path) -> Result<Database, SeedError> {
    if path.as_os_str().is_empty() || path.to_str() == Some(":memory:") {
        return Err(SeedError::InvalidState(
            "a persistent database file path is required".into(),
        ));
    }
    let is_new = !path.try_exists()?;
    if is_new {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
    } else if !path.is_file() {
        return Err(SeedError::InvalidState(
            "database path must be a file".into(),
        ));
    }
    let path = path
        .to_str()
        .ok_or_else(|| SeedError::InvalidState("database path is not valid UTF-8".into()))?;
    let environment = Environment::new()?;
    let database = environment.open(StorageLocation::OnDisk(path.into()))?;
    let pool = r2d2::Pool::builder()
        .max_size(2)
        .build(ConnectionManager::new(&database))?;
    {
        let connection = pool.get()?;
        if is_new {
            let applied = apply_migrations(&connection)?;
            tracing::info!(
                database = path,
                migrations = applied,
                "New database initialized"
            );
        } else {
            verify_migrations(&connection)?;
            tracing::info!(database = path, "Existing database migrations verified");
        }
    }
    Ok(Database { pool, is_new })
}
