use tauri::{generate_handler, ipc::Invoke};

pub mod prompt;

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![])
}

//  F: Fn(Invoke<R>) -> bool + Send + Sync + 'static,