use lingua_pick_store::UserProfile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadResult {
    pub session_id: Uuid
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginArgs {
    pub user_name: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source_language_id: u32,
    pub target_language_id: u32
}

impl From<UserProfile> for AppUserProfile {
    fn from(db: UserProfile) -> Self {
        AppUserProfile {
            id: db.id,
            user_id: db.user_id,
            source_language_id: db.source_language_id,
            target_language_id: db.target_language_id,
        }
    }
}