#![allow(warnings)]

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

    tauri::Builder::default()
        .setup_services(context)
        .setup_db().expect("Could not setup db")
        .setup_plugins()
        .setup_handlers()
        .setup_hook(setup_app)
        .build_and_run();
}
