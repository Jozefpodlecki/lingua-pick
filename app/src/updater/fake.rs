use std::{cell::RefCell, collections::VecDeque, fs::File, io::{BufReader, Read}, path::PathBuf, sync::{Arc, Mutex}, time::Duration, usize};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

use crate::updater::traits::*;
use super::traits::{UpdateProvider};

pub struct FakeUpdateBuilder {
    options: Vec<FakeUpdateOption>
}

impl FakeUpdateBuilder {

    pub fn new() -> Self {
        Self {
            options: vec![]
        }
    }

    pub fn new_binary(mut self, version: &str, path: PathBuf) -> Self {
        self.options.push(FakeUpdateOption::Binary {
            version: version.into(),
            path,
            with_total_header: true,
            delay: Duration::from_millis(100),
        });
        
        self
    }

    pub fn infinite_download(mut self, version: &str) -> Self {
        self.options.push(FakeUpdateOption::Synthetic {
            version: version.into(),
            with_total_header: true,
            total_size: 200 * 1024 * 1024,
            iterations: usize::MAX,
            delay: Duration::from_millis(100),
        });
   
        self
    }

    pub fn new_synthetic(mut self, version: &str, iterations: usize) -> Self {
        self.options.push(FakeUpdateOption::Synthetic {
            version: version.into(),
            with_total_header: true,
            total_size: 200 * 1024 * 1024,
            iterations,
            delay: Duration::from_millis(100),
        });
   
        self
    }

    pub fn latest(mut self) -> Self {
        self.options.push(FakeUpdateOption::Latest {
            
        });
   
        self
    }

    pub fn invalid_configuration(mut self) -> Self {
        self.options.push(FakeUpdateOption::Failed {
            is_fatal: true,
            error: "There was a problem with the update configuration. Please try again.".into()
        });

        self
    }

    pub fn could_not_connect_to_update_server(mut self) -> Self {
        self.options.push(FakeUpdateOption::Failed {
            is_fatal: true,
            error: "Could not connect to update server. Please check your internet connection and try again.".into()
        });

        self
    }

    pub fn failed_with_error(mut self, error: &str) -> Self {
        self.options.push(FakeUpdateOption::Failed {
            is_fatal: false,
            error: error.into()
        });

        self
    }

    pub fn failed(mut self) -> Self {
        self.options.push(FakeUpdateOption::Failed {
            is_fatal: false,
            error: "Unknown".into()
        });
   
        self
    }

    pub fn build(self) -> FakeUpdater {
        let options = self.options;

        FakeUpdater::new(options)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FakeUpdateOption {
    Idle,
    Failed {
        error: Box<str>,
        is_fatal: bool,
    },
    Latest,
    Synthetic {
        version: Box<str>,
        with_total_header: bool,
        total_size: u64,
        iterations: usize,
        delay: Duration,
    },
    Binary {
        version: Box<str>,
        path: PathBuf,
        with_total_header: bool,
        delay: Duration,
    },
}

#[derive(Debug)]
pub struct FakeUpdater {
    pub options: VecDeque<FakeUpdateOption>,
    pub current: FakeUpdateOption,
    pub has_downloaded: bool
}

impl FakeUpdater {
    pub fn new(options: Vec<FakeUpdateOption>) -> Self {
        Self {
            options: options.into(),
            current: FakeUpdateOption::Idle,
            has_downloaded: false,
        }
    }
}

#[async_trait]
impl UpdateProvider for FakeUpdater {

    fn setup(&mut self) -> Result<()> {

        if self.has_downloaded {
            return Ok(())
        }
        
        let option = self.options.pop_front().unwrap_or(FakeUpdateOption::Idle);
        self.current = option;

        match &self.current {
            FakeUpdateOption::Failed { error, is_fatal: true }  => Err(anyhow!(error.to_string())),
            _ => Ok(())
        }
    }

    async fn check(&mut self) -> Result<UpdateCheckResult> {

        if self.has_downloaded {
            return Ok(UpdateCheckResult::Downloaded)
        }

        match &self.current {
            FakeUpdateOption::Failed { error, is_fatal: false } => Ok(UpdateCheckResult::Error(error.to_string())),
            FakeUpdateOption::Latest => Ok(UpdateCheckResult::Latest),
            FakeUpdateOption::Synthetic { .. } => Ok(UpdateCheckResult::NewVersion),
            FakeUpdateOption::Binary { .. } => Ok(UpdateCheckResult::NewVersion),
            _ => Ok(UpdateCheckResult::Latest),
        }
    }

    fn version(&self) -> Option<&str> {
        match &self.current {
            FakeUpdateOption::Binary { version, .. } => {
                Some(&version)
            }
            FakeUpdateOption::Synthetic { version, .. } => {
                Some(&version)
            }
            _ => None
        }
    }

    async fn download(
        &mut self,
        mut on_chunk: ChunkCallback,
        on_finish: FinishCallback,
    ) -> Result<()> {

        match &self.current {
            FakeUpdateOption::Binary { path, with_total_header, delay, .. } => {
                let mut file = BufReader::new(File::open(path)?);
                let mut buf = vec![0u8; 8192];
                let mut data = Vec::new();
                let mut chunk_idx = 0;
                let total_size = file.get_ref().metadata()?.len();

                loop {
                    let n = file.read(&mut buf)?;
                    if n == 0 { break; }

                    data.extend_from_slice(&buf[..n]);
                    on_chunk(chunk_idx, with_total_header.then_some(total_size));
                    chunk_idx += 1;
                    sleep(*delay).await;
                }

                self.has_downloaded = true;
                on_finish();

                Ok(())
            }
            FakeUpdateOption::Synthetic { with_total_header, total_size, iterations, delay, .. } => {
                let total_size_arg = with_total_header.then_some(*total_size);
                let chunk_size = (total_size / *iterations as u64) as usize;
                let last_chunk_size = (total_size % *iterations as u64) as usize;

                let mut data = Vec::with_capacity(*total_size as usize);

                for _ in 0..*iterations {
                    data.extend(vec![0u8; chunk_size]);
                    on_chunk(chunk_size, total_size_arg);
                    sleep(*delay).await;
                }

                if last_chunk_size > 0 {
                    data.extend(vec![0u8; last_chunk_size]);
                    on_chunk(last_chunk_size, total_size_arg);
                }

                self.has_downloaded = true;
                on_finish();

                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn install(&mut self) -> Result<()> {
        std::thread::sleep(Duration::from_millis(500));

        if let FakeUpdateOption::Binary { path, .. } = &self.current {
            std::process::Command::new(path).spawn()?;
        }

        std::process::exit(0);

        Ok(())
    }
}