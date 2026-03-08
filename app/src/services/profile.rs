use futures_util::StreamExt;
use lingua_pick_store::*;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Listener, Manager};
use tokio::task;
use log::*;
use url::Url;
use uuid::Uuid;
use thiserror::Error;

use crate::{context::AppContext, services::*};

#[derive(Debug, Error)]
pub enum CreateProfileError {
    #[error("authentication failed")]
    Auth,

    #[error("failed to download resource")]
    Download,
}

type Result<T> = std::result::Result<T, CreateProfileError>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileArgs {
    pub token: String,
    pub source_language_id: u32,
    pub target_language_id: u32,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CreateProfileEvent<'a> {
    Downloading {
        kind: &'a str,
        downloaded: u32,
        file_size: u32
    },
    Error {
        kind: String,
        message: String
    }
}

pub struct ProfileManager(DependencyResolver);

impl ProfileManager {
    pub fn new(resolver: DependencyResolver, listener: AppListener) -> Self {

        {
            let resolver_cloned = resolver.clone();
            listener.listen_any("create-profile", move |event| {
                let payload = event.payload().to_owned();
                let resolver_cloned = resolver_cloned.clone();

                task::spawn(async move {
                    let emitter = resolver_cloned.get::<AppEmitter>();

                    if let Err(err) = Self::create_profile(resolver_cloned.clone(), &emitter, &payload).await {
                        error!("Could not create profile: {err}");
                        
                        let kind = match err {
                            CreateProfileError::Auth => "auth",
                            CreateProfileError::Download => "download",
                        };

                        emitter.emit(
                            "status",
                            &CreateProfileEvent::Error {
                                kind: kind.to_string(),
                                message: err.to_string(),
                            },
                        );
                    }  
                });
            });
        }
       
        Self(resolver)
    }

    async fn create_profile(resolver: DependencyResolver, emitter: &AppEmitter, payload: &str) -> Result<()> {
        let args: CreateProfileArgs = serde_json::from_str(payload)?;
        let CreateProfileArgs {
            token,
            source_language_id,
            target_language_id,
        }= args;
        let context = resolver.get::<AppContext>();
        let jwt_service = resolver.get::<JwtService>();
        let id_generator = resolver.get::<IdGenerator>();
        let profile_repo = resolver.get::<UserProfileRepository>();
        let lexeme_repo = resolver.get::<LexemeRepository>();
        let lang_asset_repo = resolver.get::<LanguageAssetsRepository>();
        let system_clock = resolver.get::<SystemClock>();
        let resource_fetcher = resolver.get::<DefaultResourceFetcher>();

        let token = jwt_service.decode(&token).map_err(|_| CreateProfileError::Auth)?;
        
        let assets = lang_asset_repo.get_by_language_id(target_language_id).map_err(|_| CreateProfileError::Download)?;
        for asset in assets {

            let uri = format!("file://C:/repos/lingua-pick/{}", asset.file_name);
            let url = Url::parse(&uri).map_err(|_| CreateProfileError::Download)?;

            let kind = {
                if asset.file_name.contains("lexemes.json") {
                    return "lexemes";
                }

                if asset.file_name.contains("graphemes.json") {
                    return "graphemes";
                }

                "unknown"
            };

            if url.scheme() == "file" && !url.to_file_path().unwrap().exists() {
                let uri = format!("https://raw.githubusercontent.com/Jozefpodlecki/lingua-pick/{}", asset.file_name);
                let url = Url::parse(&uri).map_err(|_| CreateProfileError::Download)?;

                let stream = reqwest::get(uri).await
                    .map_err(|_| CreateProfileError::Download)?
                    .bytes_stream();

                while let Some(item) = stream.next().await {
                    // println!("Chunk: {:?}", item?);

                    emitter.emit("status", &CreateProfileEvent::Downloading {
                        downloaded: 0,
                        file_size: 0,
                        kind
                    });
                }
            }



            resource_fetcher.fetch(&asset.file_name);
            // serde_json::Deserializer::from_reader
            // asset resolver?

            // lexeme_repo.bulk_insert();
        }

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

        profile_repo.create(profile).map_err(|_| CreateProfileError::Download)?;

        Ok(())
    }
}