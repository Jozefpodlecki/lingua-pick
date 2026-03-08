#![allow(dead_code)]

use anyhow::{Context, Result};
use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_updater::{Update, Updater, UpdaterExt};

use crate::updater::{ChunkCallback, FinishCallback, UpdateCheckResult};

use super::traits::{UpdateProvider};

pub struct AppUpdater {
    app_handle: AppHandle,
    data: Vec<u8>,
    updater: Option<Updater>,
    update: Option<Update>
}

impl AppUpdater {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            data: vec![],
            updater: None,
            update: None
        }
    }
}

#[async_trait]
impl UpdateProvider for AppUpdater {
    fn version(&self) -> Option<&str> {
        self.update.as_ref().map(|pr| pr.version.as_str())
    }

    async fn download(
        &mut self,
        on_chunk: ChunkCallback,
        on_finish: FinishCallback,
    ) -> Result<()> {
        let update = self.update.as_ref().with_context(|| "An error occurred whilst downloading new version")?;
        self.data = update.download(on_chunk, on_finish).await?;
        
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        let updater = self.app_handle.updater()?;
        self.updater = Some(updater);

        Ok(())
    }

    async fn check(&mut self) -> Result<UpdateCheckResult> {
        let updater = self.updater.as_ref().with_context(|| "An error occurred whilst checking updates")?;
        
        match self.update {
            Some(_) => {
                Ok(UpdateCheckResult::NewVersion)
            },
            None => {
                match updater.check().await {
                    Ok(update) => {
                        self.update = update;

                        match self.update.is_some() {
                            true => Ok(UpdateCheckResult::Downloaded),
                            false => Ok(UpdateCheckResult::Latest),
                        }
                    },
                    Err(err) => {
                        Ok(UpdateCheckResult::Error(err.to_string()))
                    },
                }
            },
        }
    }

    fn install(&mut self) -> Result<()> {
        let update = self.update.take().with_context(|| "An error occurred whilst installing new version")?;
        let data = std::mem::take(&mut self.data);
        update.install(data);

        Ok(())
    }
}