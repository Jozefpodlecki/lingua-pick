use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tauri::{AppHandle, Emitter, Manager};
use log::*;
use tokio::{spawn, sync::Mutex, time::sleep};

use crate::updater::{fake::{FakeUpdateBuilder}, plugin::AppUpdater, traits::UpdateProvider, UpdateResult};

pub struct UpdateManager {
    app_handle: AppHandle,
    provider: Mutex<Box<dyn UpdateProvider>>
}

impl UpdateManager {
    pub fn plugin(app_handle: AppHandle) -> Self {
        Self {
            app_handle: app_handle.clone(),
            provider: Mutex::new(Box::new(AppUpdater::new(app_handle)))
        }
    }

    pub fn fake(app_handle: AppHandle, builder: FakeUpdateBuilder) -> Self {
        Self {
            app_handle,
            provider: Mutex::new(Box::new(builder.build()))
        }
    }

    pub fn check(&self) {
        self.app_handle.emit("on-update", UpdateResult::Checking).unwrap();
        let app_handle = self.app_handle.clone();

        spawn(async move {
            let update_manager = app_handle.state::<UpdateManager>();
            let mut guard = update_manager.provider.lock().await;
            let provider: &mut dyn UpdateProvider = guard.as_mut();

            if let Err(error) = Self::check_inner(&app_handle, provider).await {
                error!("Could not check updates: {:?}", error);
                app_handle.emit("on-update", UpdateResult::Error { message: &error.to_string() }).unwrap();
            }
        });
    }

    async fn check_inner(app_handle: &AppHandle, provider: &mut dyn UpdateProvider) -> Result<()> {
        provider.setup()?;

        let can_update = provider.check().await?;

        match can_update {
            true => {
                let version: Arc<str> = provider.version().into();
                app_handle.emit("on-update", UpdateResult::Found { version: &version }).unwrap();
                sleep(Duration::from_millis(250)).await;

                provider.download(
                {
                    let mut downloaded = 0;
                    let version = version.clone();
                    let app_handle = app_handle.clone();
                    Box::new(move |chunk, file_size| {
                        downloaded += chunk;
                        app_handle.emit("on-update", UpdateResult::Downloading {
                            version: &version,
                            downloaded,
                            file_size,
                        }).unwrap();
                    })
                },
                {
                    let version = version.clone();
                    let app_handle = app_handle.clone();
                    Box::new(move || {
                        app_handle.emit("on-update", UpdateResult::Downloaded {
                            version: &version,
                        }).unwrap();
                    })
                }
            ).await?;
            },
            false => {
                app_handle.emit("on-update", UpdateResult::Latest).unwrap();
            },
        }

        Ok(())
    }

    pub fn install(self) -> Result<()> {

        spawn(async move {
            if let Err(err) = self.provider.lock().await.install() {
                error!("Could not install app: {:?}", err);
            }
        });

        Ok(())
    }
}