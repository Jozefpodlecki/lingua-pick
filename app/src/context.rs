use std::path::PathBuf;

use uuid::Uuid;

pub struct AppContext {
    pub current_dir: PathBuf,
    pub session_id: Uuid
}

impl AppContext {
    pub fn new() -> Self {

        let current_dir = std::env::current_dir().unwrap();
        println!("{:?}", current_dir.display().to_string());
        AppContext {
            current_dir,
            session_id: Uuid::now_v7()
        }
    }
}