use std::{io, path::PathBuf};

use thiserror::Error;

pub type Result<T> = core::result::Result<T, StoreError>;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Could not inspect database path {path}: {source}")]
    InspectPath {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Could not create database directory {path}: {source}")]
    CreateDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Could not open database {path}: {source}")]
    Open {
        path: PathBuf,
        #[source]
        source: duckdb::Error,
    },
    #[error("Database operation failed: {0}")]
    Database(#[from] duckdb::Error),
    #[error("Database connection lock is unavailable")]
    ConnectionLock,
    #[error("Database contains unknown migration version {0}")]
    UnknownMigration(u32),
    #[error("Migration {version} no longer matches its recorded name or checksum")]
    ChangedMigration { version: u32 },
    #[error("Invalid persistence record: {0}")]
    InvalidRecord(String),
    #[error("Requested record was not found")]
    NotFound,
    #[error("Session cannot be completed before every exercise has an outcome")]
    IncompleteSession,
    #[error("Stored JSON is invalid: {0}")]
    Json(#[from] serde_json::Error),
}
