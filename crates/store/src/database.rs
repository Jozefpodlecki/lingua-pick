use std::path::PathBuf;
use anyhow::Result;
use chrono::{DateTime, Utc};
use duckdb::{params, DuckdbConnectionManager, OptionalExt};
use r2d2::{ManageConnection, Pool};
use uuid::Uuid;

use crate::{migrations::apply_migrations, models::*, queries::*};

pub struct DatabaseManager(Pool<DuckdbConnectionManager>);

impl DatabaseManager {
     pub fn memory() -> Result<Self> {
        let manager = DuckdbConnectionManager::memory()?;
        
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(manager)?;

        Ok(Self(pool))
    }

    pub fn new(path: PathBuf) -> Result<Self> {
        let manager = DuckdbConnectionManager::file(path)?;
        
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(manager)?;

        Ok(Self(pool))
    }

    pub fn ensure_created(&self) -> Result<()> {

        let connection = self.0.get()?;

        apply_migrations(connection)?;

        Ok(())
    }

    pub fn pool(&self) -> &Pool<DuckdbConnectionManager> {
        &self.0
    }
}










