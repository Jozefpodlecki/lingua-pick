use lingua_pick_store::{UserProfile, UserProfileRepository};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Listener, Manager};
use tokio::task;
use log::*;
use uuid::Uuid;

use crate::services::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileArgs {
    pub token: String,
    pub source_language_id: u32,
    pub target_language_id: u32,
}

pub struct ProfileManager(AppHandle);

impl ProfileManager {
    pub fn new(app_handle: AppHandle) -> Self {

        {
            let app_handle_cloned = app_handle.clone();
            app_handle.listen_any("create-profile", move |event| {

                let args: CreateProfileArgs = {
                    let json = event.payload();
                    serde_json::from_str(json).unwrap()
                };
                let CreateProfileArgs {
                    token,
                    source_language_id,
                    target_language_id,
                }= args;
                let jwt_service = app_handle_cloned.state::<JwtService>();
                let id_generator = app_handle_cloned.state::<IdGenerator>();
                let profile_repo = app_handle_cloned.state::<UserProfileRepository>();
                let emitter = app_handle_cloned.state::<AppEmitter>();
                let system_clock = app_handle_cloned.state::<SystemClock>();

                let token = match jwt_service.decode(&token) {
                    Ok(value) => value,
                    Err(err) => {


                        return;
                    },
                };

                let user_id = token.claims.sub;
                let profile = UserProfile {
                    id: id_generator.create(),
                    user_id,
                    created_on: system_clock.now(),
                    updated_on: system_clock.now(),
                    is_active: true,
                    source_language_id,
                    target_language_id,
                };

                profile_repo.create(profile);
            });
        }
       

        Self(app_handle)
    }

    pub fn create(&self) {
        let app_handle = self.0.clone();

        task::spawn(async move {



        });
    }
}