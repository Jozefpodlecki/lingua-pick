use std::path::PathBuf;

use uuid::Uuid;

pub struct AppContext {
    pub executable_path: PathBuf,
    pub executable_dir: PathBuf,
    pub current_dir: PathBuf,
    pub remote_asset_root: &'static str,
    pub session_id: Uuid
}

impl AppContext {
    pub fn new() -> Self {

        let executable_path = std::env::current_exe().unwrap();
        let executable_dir = executable_path.parent().unwrap().to_path_buf();
        let current_dir = std::env::current_dir().unwrap();

        AppContext {
            executable_path,
            current_dir,
            executable_dir,
            remote_asset_root: "https://raw.githubusercontent.com/Jozefpodlecki/lingua-pick/main",
            session_id: Uuid::now_v7()
        }
    }
}