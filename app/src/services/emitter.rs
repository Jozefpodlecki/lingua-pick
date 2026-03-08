use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone)]
pub struct AppEmitter(AppHandle);

impl AppEmitter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(app_handle)
    }

    pub fn emit<T: ?Sized + Serialize>(&self, event: &str, value: &T) {
        self.0.emit_str(event, serde_json::to_string(value).unwrap()).unwrap();
    }
}