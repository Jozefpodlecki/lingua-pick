use std::path::PathBuf;

pub use crate::seed::DatabaseMode;
use crate::simulator::{MistakeDistribution, SimulationError};

pub struct RunnerConfig {
    pub database_path: PathBuf,
    pub database_mode: DatabaseMode,
    pub log_directory: PathBuf,
    pub user_name: String,
    pub source_language: String,
    pub target_language: String,
    pub model: String,
    pub stage: String,
    pub mistake_distribution: MistakeDistribution,
}

impl RunnerConfig {
    pub fn local() -> Result<Self, SimulationError> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        Ok(Self {
            database_path: root.join("lingua-cli.duckdb"),
            database_mode: DatabaseMode::Recreate,
            log_directory: root.join("logs"),
            user_name: "test-user".into(),
            source_language: "en-GB".into(),
            target_language: "pt-BR".into(),
            // model: "meta/muse-glimmer".into(),
            model: "qwen/qwen3-vl-8b".into(),
            stage: "words".into(),
            mistake_distribution: MistakeDistribution::new(25)?
                .with_rate("match_words", 30)?
                .with_rate("transliterate", 35)?,
        })
    }
}
