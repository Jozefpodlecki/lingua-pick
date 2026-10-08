use thiserror::Error;

use crate::{migration::MigrationError, store::StoreError};

#[derive(Debug, Error)]
pub enum SeedError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid saved or requested seed state: {0}")]
    InvalidState(String),

    #[error("database error: {0}")]
    Database(#[from] duckdb_neo::error::Error),

    #[error("database pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("migration error: {0}")]
    Migration(#[from] MigrationError),

    #[error("store error: {0}")]
    Store(#[from] StoreError),

    #[error("password hash error: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),

    #[error("seed language not found: {0}")]
    LanguageNotFound(String),
}
