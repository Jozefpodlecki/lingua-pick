use anyhow::Result;
use duckdb::{params, DuckdbConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{queries::*, Session};

pub struct UserSkillMetricRepository(Pool<DuckdbConnectionManager>);

impl UserSkillMetricRepository {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Self {
        Self(pool)
    }
}