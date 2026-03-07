use lingua_pick_store::*;
use tauri::{App, Wry};

use crate::{context::AppContext, handlers::generate_handlers, notifier::SetupEndedNotifier, services::*, setup::setup_app};

pub trait BuilderExtensions {
    fn setup_hook<F>(self, hook: F) -> Self
        where
            Self: Sized,
            F: FnOnce(&mut App<Wry>) -> std::result::Result<(), Box<dyn std::error::Error>> + Send + 'static;
    fn setup_handlers(self) -> Self where Self: Sized;
    fn setup_services(self, context: AppContext) -> Self where Self: Sized;
    fn setup_plugins(self) -> Self where Self: Sized;
    fn setup_db(self) -> anyhow::Result<Self> where Self: Sized;
    fn build_and_run(self);
}

impl BuilderExtensions for tauri::Builder<Wry> {

    fn setup_hook<F>(self, hook: F) -> Self
        where
        Self: Sized,
        F: FnOnce(&mut App<Wry>) -> std::result::Result<(), Box<dyn std::error::Error>> + Send + 'static
    {
        let mut builder = self;

        builder = builder.setup(hook);

        builder
    }

    fn setup_handlers(self) -> Self {
        let mut builder = self;

        builder = builder.invoke_handler(generate_handlers());

        builder
    }

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
            .manage(SessionRepository::new(pool.clone()))
            .manage(LexemeRepository::new(pool.clone()))
            .manage(ExerciseRepository::new(pool.clone()))
            .manage(CharacterRepository::new(pool.clone()))
            .manage(UserStatRepository::new(pool.clone()))
            .manage(UserSkillMetricRepository::new(pool.clone()));

        Ok(builder)
    }
    
    fn setup_plugins(self) -> Self {
        let mut builder = self;

        builder = builder.plugin(tauri_plugin_clipboard_manager::init())
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
            .plugin(tauri_plugin_opener::init());

        builder
    }
    
    fn setup_services(self, context: AppContext) -> Self {
         let mut builder = self;

        builder = builder
            .manage(SetupEndedNotifier::new())
            .manage(context)
            .manage(IdGenerator::new())
            .manage(SystemClock::new())
            .manage(AppPasswordHasher::new())
            .manage(JwtService::new().expect("Fatal"))
            .manage(TranscriptionService::new().expect("Fatal"))
            .manage(AudioManager::new().expect("Fatal"));

        builder
    }
    
    fn build_and_run(self) {
        let context = tauri::generate_context!();
        let app = self.build(context).expect("error while running tauri application");
        app.run(|_, _| {});
    }
}
