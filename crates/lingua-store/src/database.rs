use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use duckdb::Connection;

use crate::{Result, SessionRepository, StatsRepository, StoreError, migrations::apply_migrations};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreInitialization {
    pub created_database: bool,
    pub applied_migrations: usize,
}

pub struct LinguaStore {
    connection: Mutex<Connection>,
    path: Option<PathBuf>,
    initialization: StoreInitialization,
}

impl LinguaStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let existed = path
            .try_exists()
            .map_err(|source| StoreError::InspectPath {
                path: path.clone(),
                source,
            })?;

        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            fs::create_dir_all(parent).map_err(|source| StoreError::CreateDirectory {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let mut connection = Connection::open(&path).map_err(|source| StoreError::Open {
            path: path.clone(),
            source,
        })?;
        let applied_migrations = apply_migrations(&mut connection)?;

        Ok(Self {
            connection: Mutex::new(connection),
            path: Some(path),
            initialization: StoreInitialization {
                created_database: !existed,
                applied_migrations,
            },
        })
    }

    pub fn in_memory() -> Result<Self> {
        let mut connection = Connection::open_in_memory()?;
        let applied_migrations = apply_migrations(&mut connection)?;

        Ok(Self {
            connection: Mutex::new(connection),
            path: None,
            initialization: StoreInitialization {
                created_database: true,
                applied_migrations,
            },
        })
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub const fn initialization(&self) -> StoreInitialization {
        self.initialization
    }

    pub fn sessions(&self) -> SessionRepository<'_> {
        SessionRepository::new(self)
    }

    pub fn stats(&self) -> StatsRepository<'_> {
        StatsRepository::new(self)
    }

    pub(crate) fn connection(&self) -> Result<MutexGuard<'_, Connection>> {
        self.connection
            .lock()
            .map_err(|_| StoreError::ConnectionLock)
    }
}
