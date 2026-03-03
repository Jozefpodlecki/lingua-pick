use uuid::Uuid;

pub struct AppContext {
    pub session_id: Uuid
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            session_id: Uuid::now_v7()
        }
    }
}