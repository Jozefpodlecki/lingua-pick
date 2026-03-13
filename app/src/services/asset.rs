use anyhow::Result;
use futures_util::StreamExt;
use lingua_pick_store::*;
use tokio::{fs::File, io::AsyncWriteExt};
use crate::{context::AppContext, services::*};

pub struct AssetResolver(DependencyResolver);

impl AssetResolver {
    pub fn new(resolver: DependencyResolver) -> Self {
        Self(resolver)
    }

    pub async fn resolve(&self, asset: LanguageAsset) -> Result<()> {
        let resolver = &self.0;
        let context = resolver.get::<AppContext>();
        let resource_fetcher = resolver.get::<DefaultResourceFetcher>();
        let lexeme_repo = resolver.get::<LexemeRepository>();
        let lang_asset_repo = resolver.get::<LanguageAssetsRepository>();
        let emitter = resolver.get::<AppEmitter>();
        let file_path = context.current_dir.join(&*asset.file_name);

        if !file_path.exists() {
            let uri = format!("{}/{}", context.remote_asset_root, asset.file_name);

            let mut stream = reqwest::get(uri)
                .await
                .map_err(|_| CreateProfileError::Download)?
                .bytes_stream();

            let mut writer = File::create(file_path)
                .await
                .map_err(|_| CreateProfileError::Download)?;

            while let Some(result) = stream.next().await {
                let bytes = result?;
                emitter.emit("status", &CreateProfileEvent::Downloading {
                    downloaded: 0,
                    file_size: 0,
                    kind: asset.kind()
                });

                writer.write(&bytes).await?;
            }
        }

        // let uri = format!("file://{}/{}", context.current_dir.display(), asset.file_name);
        //     let url = Url::parse(&uri).map_err(|_| CreateProfileError::Download)?;

        //     if url.scheme() == "file" && !url.to_file_path().unwrap().exists() {
        //         let uri = format!("https://raw.githubusercontent.com/Jozefpodlecki/lingua-pick/{}", asset.file_name);
        //         let url = Url::parse(&uri).map_err(|_| CreateProfileError::Download)?;

        //         let path = context.current_dir.join(&*asset.file_name);



        //         while let Some(item) = stream.next().await {
        //             // println!("Chunk: {:?}", item?);

        //             emitter.emit("status", &CreateProfileEvent::Downloading {
        //                 downloaded: 0,
        //                 file_size: 0,
        //                 kind: asset.kind()
        //             });
        //         }
        //     }



            // resource_fetcher.fetch(&asset.file_name);
            // serde_json::Deserializer::from_reader
            // asset resolver?

            // lexeme_repo.bulk_insert();

        Ok(())
    }
}