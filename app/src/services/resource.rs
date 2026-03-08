use anyhow::Result;
use async_trait::async_trait;
use tokio::fs;
use url::Url;

#[async_trait]
pub trait ResourceFetcher {
    async fn fetch(&self, uri: &str) -> Result<String>;
}

pub struct DefaultResourceFetcher;

#[async_trait]
impl ResourceFetcher for DefaultResourceFetcher {
    async fn fetch(&self, uri: &str) -> Result<String> {
        let url = Url::parse(uri)?;
        match url.scheme() {
            "file" => {
                let path = url.to_file_path().map_err(|_| anyhow::anyhow!("Invalid file path"))?;
                Ok(fs::read_to_string(path).await?)
            },
            "http" | "https" => {
                let body = reqwest::get(uri).await?.text().await?;
                Ok(body)
            },
            other => Err(anyhow::anyhow!("Unsupported scheme: {}", other)),
        }
    }
}

impl DefaultResourceFetcher {
    pub fn new() -> Self {
        Self
    }
}