
use std::{error::Error};
use anyhow::Result;
use tauri::{App, AppHandle, Listener, Manager};

use crate::{notifier::SetupEndedNotifier, updater::setup_updater};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.handle();
    let starter = app_handle.get_webview_window("starter").unwrap();
    starter.center()?;

    setup_updater(app_handle);

    let notifier = app_handle.state::<SetupEndedNotifier>();
    notifier.notify_loaded();

    Ok(())
}