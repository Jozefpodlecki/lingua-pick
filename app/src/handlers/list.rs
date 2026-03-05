use tauri::ipc::Invoke;
use tauri::generate_handler;

use crate::handlers::*;

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
        load_app,
        login_with_creds,
        login_with_windows,
        get_current_profile
    ])
}
