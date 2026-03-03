use crate::{context::AppContext, handlers::generate_handlers, notifier::SetupEndedNotifier, setup::setup_app};

mod setup;
mod context;
mod handlers;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = AppContext::new();

    tauri::Builder::default()
        .manage(SetupEndedNotifier::new())
        .manage(context)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stdout,
            ))
            .build())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handlers())
        .setup(setup_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
