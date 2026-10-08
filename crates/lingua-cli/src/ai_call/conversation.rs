use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use lingua_ai::{
    ContentPart, FinishReasonUnified, LlmClient, MessageContent, ModelMessage, Role, ToolChoice,
};
use tracing::Instrument;
use uuid::Uuid;

use crate::tools::GenerationTools;

const MAX_TURNS: usize = 6;
const MAX_TOOL_CALLS: usize = 12;
const TIMEOUT: Duration = Duration::from_secs(30 * 60);

pub async fn generate(
    client: &LlmClient,
    model: &str,
    instructions: &str,
    prompt: &str,
    tools: &mut GenerationTools,
) -> Result<String, String> {
    let call_id = Uuid::now_v7();
    async {
        let started = Instant::now();
        let mut messages = vec![ModelMessage::user(prompt)];
        let definitions = tools.definitions();
        let mut call_ids = BTreeSet::new();
        let mut calls = 0;
        let mut logged_messages = messages.len();
        tracing::debug!(system_prompt = instructions, user_prompt = prompt,
            tool_definitions = %serde_json::to_string_pretty(&definitions).map_err(|error| error.to_string())?, "AI conversation request");

        for turn in 1..=MAX_TURNS {
            let remaining = TIMEOUT.checked_sub(started.elapsed())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| "AI conversation timeout exceeded".to_string())?;
            let final_turn = turn == MAX_TURNS || calls == MAX_TOOL_CALLS;
            if final_turn {
                messages.push(ModelMessage::user("The tool budget is exhausted. Return exactly one final exercise JSON object using only supplied slots and concepts already retrieved. Do not request more tools."));
            }
            let choice = if final_turn { ToolChoice::None } else { ToolChoice::Auto };
            let turn_started = Instant::now();
            tracing::info!(turn, tools_enabled = !final_turn, "AI turn started");
            if messages.len() > logged_messages {
                tracing::debug!(turn, conversation_updates = %serde_json::to_string_pretty(&messages[logged_messages..]).map_err(|error| error.to_string())?, "AI conversation continued");
            }
            logged_messages = messages.len();
            let response = client.generate_turn(model, instructions, messages.clone(),
                definitions.clone(), choice, remaining.as_millis() as u64)
                .await.map_err(|error| {
                    tracing::error!(turn, error = %error, "AI turn failed");
                    error.to_string()
                })?;
            tracing::debug!(turn, raw_response = %response.text,
                tool_calls = %serde_json::to_string_pretty(&response.tool_calls).map_err(|error| error.to_string())?, "AI turn response");
            tracing::info!(turn, elapsed_ms = turn_started.elapsed().as_millis() as u64,
                tool_calls = response.tool_calls.len(), finish_reason = ?response.finish_reason.unified, "AI turn completed");
            if response.tool_calls.is_empty() {
                if response.finish_reason.unified != FinishReasonUnified::Stop || response.text.trim().is_empty() {
                    return Err(format!("AI turn did not return a complete final response: {:?}", response.finish_reason.unified));
                }
                return Ok(response.text);
            }
            if final_turn {
                return Err("model requested tools after tools were disabled".into());
            }
            if !matches!(response.finish_reason.unified, FinishReasonUnified::Stop | FinishReasonUnified::ToolCalls) {
                return Err("model returned an incomplete tool-call turn".into());
            }
            if response.tool_calls.len() > MAX_TOOL_CALLS - calls {
                return Err("model requested more tools than the remaining call budget".into());
            }
            if response.response_messages.is_empty() {
                return Err("tool response has no assistant conversation message".into());
            }
            messages.extend(response.response_messages);
            for call in response.tool_calls {
                if call.tool_call_id.trim().is_empty() || !call_ids.insert(call.tool_call_id.clone()) {
                    return Err("tool calls require distinct nonblank call IDs".into());
                }
                let tool_started = Instant::now();
                tracing::debug!(turn, tool_call_id = %call.tool_call_id, tool = %call.tool_name,
                    arguments = %serde_json::to_string_pretty(&call.input).map_err(|error| error.to_string())?, "Store tool requested");
                calls += 1;
                let remaining = TIMEOUT.checked_sub(started.elapsed())
                    .ok_or_else(|| "AI conversation timeout exceeded".to_string())?;
                let output = tokio::time::timeout(remaining, tools.execute(call.clone())).await
                    .map_err(|_| "AI conversation timeout exceeded during tool execution".to_string())?;
                tracing::debug!(turn, tool_call_id = %call.tool_call_id, tool = %call.tool_name,
                    result = %serde_json::to_string_pretty(&output.value).map_err(|error| error.to_string())?, "Store tool result");
                if output.is_error {
                    tracing::warn!(turn, tool = %call.tool_name, "Store tool returned an error");
                }
                tracing::info!(turn, tool = %call.tool_name, tool_call_id = %call.tool_call_id,
                    elapsed_ms = tool_started.elapsed().as_millis() as u64, is_error = output.is_error, "Store tool completed");
                let part = ContentPart::ToolResult {
                    tool_call_id: call.tool_call_id, tool_name: Some(call.tool_name),
                    result: output.value, is_error: Some(output.is_error),
                    preliminary: None, dynamic: None, provider_options: None,
                };
                messages.push(ModelMessage { role: Role::Tool, content: MessageContent::Parts(vec![part]) });
            }
        }
        Err("AI conversation turn budget exhausted".into())
    }.instrument(tracing::info_span!("ai_call", %call_id, stage = "generation", model)).await
}
