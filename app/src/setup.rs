
use std::{error::Error};
use ananke_lib::ProjectRepository;
use anyhow::Result;
use tauri::{App, AppHandle, Listener, Manager};

use crate::{background::BackgroundWorker, notifier::SetupEndedNotifier, updater::setup_updater};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.handle();
    let starter = app_handle.get_webview_window("starter").unwrap();
    starter.center()?;

    setup_updater(app_handle);

    let mut background = BackgroundWorker::new();
    background.run()?;
    app_handle.manage(background);

    let projects = ProjectRepository::new();
    app_handle.manage(projects);

    let notifier = app_handle.state::<SetupEndedNotifier>();
    notifier.notify_loaded();

    Ok(())
}