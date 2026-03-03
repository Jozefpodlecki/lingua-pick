use tauri::ipc::Invoke;
use tauri::generate_handler;

use crate::handlers::*;

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
        load_app,
        get_recent_projects,
        analyse_binary
    ])
}
