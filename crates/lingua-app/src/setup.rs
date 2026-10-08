use lingua_ai::LlmClient;
// use lingua_store::LinguaStore;
use tauri::{App, Builder, EventLoopMessage, Manager, Wry};

pub fn register_dependencies(builder: Builder<Wry>) -> Builder<Wry> {
    builder.manage(LlmClient::new())
}

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    
    let database_path = app.path().app_data_dir()?.join("lingua-pick.duckdb");
    // let store = LinguaStore::open(database_path)?;
    // app.manage(store);

    Ok(())

}