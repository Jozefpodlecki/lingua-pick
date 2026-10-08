use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    sync::Mutex,
};

use chrono::Utc;
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    prelude::*,
};
use uuid::Uuid;

mod readable;

pub fn init(
    directory: &Path,
    run_id: Uuid,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    std::fs::create_dir_all(directory)?;
    let path = directory.join(format!(
        "run-{}-{run_id}.log",
        Utc::now().format("%Y%m%dT%H%M%SZ")
    ));
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    let file_layer = tracing_subscriber::fmt::layer()
        .event_format(readable::Readable)
        .with_ansi(false)
        .with_writer(Mutex::new(file))
        .with_filter(
            Targets::new()
                .with_default(LevelFilter::WARN)
                .with_target("lingua_cli", LevelFilter::DEBUG),
        );
    let console_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_filter(
            Targets::new()
                .with_default(LevelFilter::WARN)
                .with_target("lingua_cli", LevelFilter::INFO),
        );
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .try_init()?;
    tracing::info!(%run_id, log_file = %path.display(), "File logging initialized");
    Ok(path)
}
