
use std::{error::Error};
use anyhow::Result;
use tauri::{App, Manager};

use crate::{notifier::SetupEndedNotifier, services::*, updater::setup_updater};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.handle();
    let starter = app_handle.get_webview_window("starter").unwrap();
    starter.center()?;

    let resolver = DependencyResolver::new(app_handle.clone());
    let emitter = AppEmitter::new(app_handle.clone());
    let listener = AppListener::new(app_handle.clone());
    app_handle.manage(resolver.clone());
    app_handle.manage(AssetResolver::new(resolver.clone()));
    app_handle.manage(ProfileManager::new(resolver, listener.clone()));
    app_handle.manage(emitter);
    app_handle.manage(listener);
    
    setup_updater(app_handle);

    let notifier = app_handle.state::<SetupEndedNotifier>();
    notifier.notify_loaded();

    Ok(())
}