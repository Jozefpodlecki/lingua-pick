use anyhow::Result;

pub type ChunkCallback = Box<dyn FnMut(usize, Option<u64>) + Send>;
pub type FinishCallback = Box<dyn FnOnce() + Send>;

#[async_trait::async_trait]
pub trait UpdateProvider: Send + Sync + 'static {
    fn version(&self) -> &str;
    async fn download(
        &mut self,
        on_chunk: ChunkCallback,
        on_finish: FinishCallback,
    ) -> Result<()>;
    fn setup(&mut self) -> Result<()>;
    async fn check(&mut self) -> Result<bool>;
    fn install(&mut self) -> Result<()>;
}