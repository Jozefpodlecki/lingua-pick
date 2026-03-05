use lingua_pick_store::*;
use tauri::Runtime;

use crate::{context::AppContext, handlers::generate_handlers, notifier::SetupEndedNotifier, services::*, setup::setup_app};

mod setup;
mod context;
mod handlers;
mod notifier;
mod services;
mod updater;

pub trait BuilderExtensions {
    fn setup_db(self) -> anyhow::Result<Self> where Self: Sized;
}

impl<R: Runtime> BuilderExtensions for tauri::Builder<R> {
    fn setup_db(self) -> anyhow::Result<Self> {
        
        let path = "lingua-pick.duckdb";
        let database = DatabaseManager::new(path.into())?;
        database.ensure_created()?;

        let pool = database.pool().clone();
        
        let builder = self
            .manage(database)
            .manage(UserRepository::new(pool.clone()))
            .manage(UserProfileRepository::new(pool.clone()))
            .manage(LanguageRepository::new(pool.clone()))
            .manage(SessionRepository::new(pool.clone()));
            // .manage(UserRepository::new(pool.clone()))

        Ok(builder)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = AppContext::new();

    tauri::Builder::default()
        .manage(SetupEndedNotifier::new())
        .manage(context)
        .manage(AppPasswordHasher::new())
        .manage(JwtService::new())
        .manage(TranscriptionService::new().expect("Fatal"))
        .manage(AudioManager::new().expect("Fatal"))
        .setup_db().expect("Test")
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stdout,
            ))
            // .filter(filter)
            .level_for("tao", log::LevelFilter::Error)
            .build())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handlers())
        // .channel_interceptor(|_, _, _, _| {
        //     true
        // })
        .setup(setup_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
