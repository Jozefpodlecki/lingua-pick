use aimux_core::prelude::*;
pub use aimux_core::{
    content::ContentPart,
    message::{MessageContent, ModelMessage, Role},
    result::StreamTextResultAggregated,
    tool::{FunctionTool, Tool, ToolCall, ToolChoice},
    types::FinishReasonUnified,
};
use aimux_providers::lmstudio::{LmStudioConfig, LmStudioProvider};
use alloc::{
    string::String,
    vec::{self, Vec},
};

pub struct LlmClient {
    provider: LmStudioProvider,
}

impl LlmClient {
    pub async fn generate_turn(
        &self,
        model: &str,
        instructions: &str,
        messages: Vec<ModelMessage>,
        tools: Vec<Tool>,
        tool_choice: ToolChoice,
        remaining_ms: u64,
    ) -> Result<StreamTextResultAggregated, AiMuxError> {
        let model = self.provider.model(model);
        let options = GenerateTextOptions {
            instructions: Some(instructions.into()),
            tools: Some(tools),
            tool_choice: Some(tool_choice),
            max_output_tokens: Some(4096),
            max_retries: Some(0),
            timeout: Some(aimux_core::options::TimeoutConfiguration {
                total_ms: Some(remaining_ms),
                first_chunk_ms: Some(remaining_ms.min(10 * 60 * 1000)),
                chunk_ms: Some(remaining_ms.min(5 * 60 * 1000)),
                ..Default::default()
            }),
            ..Default::default()
        };
        stream_text(&model, messages, options)
            .await?
            .consume()
            .await
    }

    pub async fn generate_streamed(
        &self,
        model: &str,
        instructions: &str,
        prompt: &str,
    ) -> Result<String, AiMuxError> {
        let model = self.provider.model(model);
        let options = GenerateTextOptions {
            instructions: Some(instructions.into()),
            max_output_tokens: Some(4096),
            max_retries: Some(0),
            timeout: Some(aimux_core::options::TimeoutConfiguration {
                total_ms: Some(30 * 60 * 1000),
                first_chunk_ms: Some(10 * 60 * 1000),
                chunk_ms: Some(5 * 60 * 1000),
                ..Default::default()
            }),
            ..Default::default()
        };

        stream_text(&model, prompt, options).await?.text().await
    }

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

        let result = generate_text(&model, prompt, options).await?;

        Ok(result.text)
    }
}
