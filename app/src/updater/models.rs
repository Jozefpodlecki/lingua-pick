use serde::Serialize;

pub type ChunkCallback = Box<dyn FnMut(usize, Option<u64>) + Send>;
pub type FinishCallback = Box<dyn FnOnce() + Send>;

pub enum UpdateCheckResult {
    Latest,
    Downloaded,
    NewVersion,
    Error(String)
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UpdateResult<'a> {
    Idle,
    Checking,
    Found {
        version: &'a str,
    },
    Downloading {
        version: &'a str,
        downloaded: usize,
        #[serde(rename = "fileSize")] 
        file_size: Option<u64>
    },
    Downloaded {
        version: &'a str,
    },
    Latest,
    Error {
        message: &'a str
    }
}
