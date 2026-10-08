use std::time::Instant;

use lingua_ai::LlmClient;
use tracing::Instrument;
use uuid::Uuid;

pub mod conversation;

pub async fn generate(
    client: &LlmClient,
    model: &str,
    stage: &str,
    instructions: &str,
    prompt: &str,
) -> Result<String, String> {
    let call_id = Uuid::now_v7();
    async {
        tracing::debug!(system_prompt = instructions, user_prompt = prompt, "AI request");
        tracing::info!("AI request started");
        let started = Instant::now();
        match client.generate_streamed(model, instructions, prompt).await {
            Ok(response) => {
                tracing::debug!(raw_response = %response, "AI response");
                tracing::info!(elapsed_ms = started.elapsed().as_millis() as u64, response_bytes = response.len(), "AI request completed");
                Ok(response)
            }
            Err(error) => {
                tracing::error!(elapsed_ms = started.elapsed().as_millis() as u64, error = %error, "AI request failed");
                Err(error.to_string())
            }
        }
    }.instrument(tracing::info_span!("ai_call", %call_id, stage, model)).await
}
