use tauri::{Builder, EventLoopMessage, Wry};

use crate::context::AppContext;
use crate::builder::BuilderExtensions;
use crate::setup::setup_app;

mod setup;
mod context;
mod handlers;
mod notifier;
mod services;
mod updater;
mod builder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = AppContext::new();

    // let test: Builder<Wry> = tauri::Builder::default();

    tauri::Builder::default()
        .setup_services(context)
        .setup_db().expect("Test")
        .setup_plugins()
        .setup_handlers()
        // .channel_interceptor(|_, _, _, _| {
        //     true
        // })
        .setup_hook(setup_app)
        .build_and_run();
}
