use serde::Serialize;
use tauri::{AppHandle, Listener, Manager};
use log::*;

use crate::updater::{fake::FakeUpdateBuilder, manager::{UpdateManager}};

pub fn setup_updater(app_handle: &AppHandle) {

    let builder = FakeUpdateBuilder::new()
        .new_synthetic("1.0.0", 100)
        .latest();
    let update_manager = UpdateManager::fake(app_handle.clone(), builder);
    app_handle.manage(update_manager);

    let app_handle = app_handle.clone();
    let check_app_handle = app_handle.clone();

    app_handle.clone().listen_any("check_updates", move |event| {
        let app_handle = check_app_handle.clone();
        let update_manager = app_handle.state::<UpdateManager>();

        update_manager.check()
    });

    app_handle.clone().listen_any("install", move |event| {
        unsafe {
            #[allow(deprecated)]
            let update_manager = app_handle.unmanage::<UpdateManager>().unwrap_unchecked();

            app_handle.unlisten(event.id());

            update_manager.install().expect("Could not install app");
        }
    });
}

