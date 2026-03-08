use serde::Serialize;
use tauri::{AppHandle, Event, EventId, Listener};

#[derive(Clone)]
pub struct AppListener(AppHandle);

impl AppListener {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(app_handle)
    }

    pub fn listen_any<F>(&self, event: impl Into<String>, handler: F) -> EventId
        where F: Fn(Event) + Send + 'static
        {
        self.0.listen_any(event, handler)
    }
}