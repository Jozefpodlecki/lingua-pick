
use std::{error::Error};
use anyhow::Result;
use tauri::{App, Manager};

use crate::{notifier::SetupEndedNotifier, services::*, updater::setup_updater};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.handle();
    let starter = app_handle.get_webview_window("starter").unwrap();
    starter.center()?;

    app_handle.manage(ProfileManager::new(app_handle.clone()));
    app_handle.manage(AppEmitter::new(app_handle.clone()));
    app_handle.manage(DependencyResolver::new(app_handle.clone()));
    setup_updater(app_handle);

    let notifier = app_handle.state::<SetupEndedNotifier>();
    notifier.notify_loaded();

    Ok(())
}