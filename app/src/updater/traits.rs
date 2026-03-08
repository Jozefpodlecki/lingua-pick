use anyhow::Result;

use crate::updater::{ChunkCallback, FinishCallback, UpdateCheckResult};
#[async_trait::async_trait]
pub trait UpdateProvider: Send + Sync + 'static {
    fn version(&self) -> Option<&str>;
    async fn download(
        &mut self,
        on_chunk: ChunkCallback,
        on_finish: FinishCallback,
    ) -> Result<()>;
    fn setup(&mut self) -> Result<()>;
    async fn check(&mut self) -> Result<UpdateCheckResult>;
    fn install(&mut self) -> Result<()>;
}