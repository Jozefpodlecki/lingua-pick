use aimux_core::prelude::*;
use aimux_providers::lmstudio::{LmStudioConfig, LmStudioProvider};
use alloc::{string::String, vec::{self, Vec}};

pub struct LlmClient {
    provider: LmStudioProvider,
}

impl LlmClient {
    pub fn new() -> Self {
        let config = LmStudioConfig::new("lm-studio");
        let provider = LmStudioProvider::new(config);

        Self { provider }
    }

    pub async fn get_models(&self) -> Result<Vec<RuntimeModel>, AiMuxError> {
        let models: Vec<RuntimeModel> = self.provider.list_models().await?;

        Ok(models)
    }

    pub async fn generate(
        &self,
        model: &str,
        instructions: &str,
        prompt: &str,
    ) -> Result<String, AiMuxError> {
        let model = self.provider.model(model);

        let options = GenerateTextOptions {
            instructions: Some(instructions.into()),
            ..Default::default()
        };

        let result = generate_text(
            &model,
            prompt,
            options,
        )
        .await?;

        Ok(result.text)
    }
}