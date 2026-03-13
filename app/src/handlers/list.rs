use tauri::ipc::Invoke;
use tauri::generate_handler;

use crate::handlers::*;

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
        load_app,
        login_with_creds,
        login_with_windows,
        get_current_profile,
        get_all_profiles,
        get_all_languages,
        get_default_language,
        start_recording,
        stop_recording,
        get_session,
        get_exercise,
        get_asset_dir
    ])
}
