use std::path::Path;

pub use error::SeedError;
pub use stores::SeedResult;

mod database;
mod error;
mod state;
mod stores;

pub fn generate(
    user_name: &str,
    source_language: &str,
    target_language: &str,
) -> Result<SeedResult, SeedError> {
    open(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lingua-cli.duckdb"),
        user_name,
        source_language,
        target_language,
    )
}

pub fn open(
    path: impl AsRef<Path>,
    user_name: &str,
    source_language: &str,
    target_language: &str,
) -> Result<SeedResult, SeedError> {
    if user_name.trim().is_empty()
        || source_language.split('-').next() != Some("en")
        || target_language.trim().is_empty()
    {
        return Err(SeedError::InvalidState(
            "a learner name, English source and target are required".into(),
        ));
    }
    let database = database::open(path.as_ref())?;
    let state = if database.is_new {
        state::create(&database.pool, user_name, source_language, target_language)?
    } else {
        state::resolve(&database.pool, user_name, source_language, target_language)?
    };
    Ok(stores::resolve(database.pool, state))
}
