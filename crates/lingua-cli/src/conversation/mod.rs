mod error;

use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use lingua_ai::{
    ContentPart, FinishReasonUnified, LlmClient, MessageContent, ModelMessage, Role, ToolCall,
    ToolChoice,
};
use tracing::{Instrument, debug, info, warn};
use uuid::Uuid;

use crate::tools::GenerationTools;

pub use error::ConversationError;

type Result<T> = std::result::Result<T, ConversationError>;

const MAX_TOOL_CALLS: usize = 12;
const MAX_TURNS: usize = MAX_TOOL_CALLS + 1;
const TIMEOUT: Duration = Duration::from_secs(30 * 60);

const FINAL_INSTRUCTIONS: &str = "\
No further tool calls are available. Return exactly one final exercise JSON \
object using only supplied slots and concepts already retrieved. \
Do not request more tools.";

pub async fn generate(
    client: &LlmClient,
    model: &str,
    instructions: &str,
    prompt: &str,
    tools: &mut GenerationTools,
) -> Result<String> {
    Conversation::new(prompt)
        .run(client, model, instructions, tools)
        .instrument(tracing::info_span!(
            "conversation",
            call_id = %Uuid::now_v7(),
            model
        ))
        .await
}

struct Conversation {
    messages: Vec<ModelMessage>,
    call_ids: BTreeSet<String>,
    tool_calls: usize,
    started: Instant,
}

impl Conversation {
    fn new(prompt: &str) -> Self {
        Self {
            messages: vec![ModelMessage::user(prompt)],
            call_ids: BTreeSet::new(),
            tool_calls: 0,
            started: Instant::now(),
        }
    }

    async fn run(
        mut self,
        client: &LlmClient,
        model: &str,
        instructions: &str,
        tools: &mut GenerationTools,
    ) -> Result<String> {
        let definitions = tools.definitions();

        debug!(system_prompt = instructions, "generation instructions");

        for turn in 1..=MAX_TURNS {
            let final_turn = turn == MAX_TURNS || self.tool_calls == MAX_TOOL_CALLS;

            if final_turn {
                let reason = if self.tool_calls == MAX_TOOL_CALLS {
                    "tool_call_limit"
                } else {
                    "turn_limit"
                };

                info!(
                    turn,
                    reason,
                    tool_calls_used = self.tool_calls,
                    max_tool_calls = MAX_TOOL_CALLS,
                    max_turns = MAX_TURNS,
                    "forcing final generation response"
                );
                debug!(
                    turn,
                    user_prompt = FINAL_INSTRUCTIONS,
                    "conversation addition"
                );
                self.messages.push(ModelMessage::user(FINAL_INSTRUCTIONS));
            }

            let choice = if final_turn {
                ToolChoice::None
            } else {
                ToolChoice::Auto
            };

            let remaining = self.remaining()?;
            let started = Instant::now();

            let response = tokio::time::timeout(
                remaining,
                client.generate_turn(
                    model,
                    instructions,
                    self.messages.clone(),
                    definitions.clone(),
                    choice,
                    remaining.as_millis() as u64,
                ),
            )
            .await
            .map_err(|_| ConversationError::Timeout)?
            .map_err(|error| ConversationError::Model {
                turn,
                message: error.to_string(),
            })?;

            info!(
                turn,
                elapsed_ms = started.elapsed().as_millis() as u64,
                tool_calls = response.tool_calls.len(),
                finish_reason = ?response.finish_reason.unified,
                "model turn completed"
            );

            debug!(
                turn,
                raw_response = %response.text,
                "generation model response"
            );

            if response.tool_calls.is_empty() {
                return Self::final_response(response.text, response.finish_reason.unified);
            }

            if final_turn {
                return Err(ConversationError::ToolsDisabled);
            }

            if !matches!(
                response.finish_reason.unified,
                FinishReasonUnified::Stop | FinishReasonUnified::ToolCalls
            ) {
                return Err(ConversationError::IncompleteToolCalls {
                    reason: response.finish_reason.unified,
                });
            }

            let remaining_calls = MAX_TOOL_CALLS - self.tool_calls;

            if response.tool_calls.len() > remaining_calls {
                return Err(ConversationError::ToolBudgetExceeded {
                    requested: response.tool_calls.len(),
                    remaining: remaining_calls,
                });
            }

            if response.response_messages.is_empty() {
                return Err(ConversationError::MissingAssistantMessage);
            }

            self.messages.extend(response.response_messages);

            for call in response.tool_calls {
                self.execute_tool(turn, call, tools).await?;
            }
        }

        Err(ConversationError::TurnBudgetExceeded { max: MAX_TURNS })
    }

    async fn execute_tool(
        &mut self,
        turn: usize,
        call: ToolCall,
        tools: &mut GenerationTools,
    ) -> Result<()> {
        if call.tool_call_id.trim().is_empty() {
            return Err(ConversationError::EmptyToolCallId);
        }

        if !self.call_ids.insert(call.tool_call_id.clone()) {
            return Err(ConversationError::DuplicateToolCallId {
                id: call.tool_call_id,
            });
        }

        let started = Instant::now();

        debug!(
            turn,
            tool = %call.tool_name,
            tool_call_id = %call.tool_call_id,
            arguments = %call.input,
            "executing tool"
        );

        let output = tokio::time::timeout(self.remaining()?, tools.execute(call.clone()))
            .await
            .map_err(|_| ConversationError::ToolTimeout {
                tool: call.tool_name.clone(),
            })?;

        self.tool_calls += 1;

        debug!(
            turn,
            tool = %call.tool_name,
            tool_call_id = %call.tool_call_id,
            result = %output.value,
            is_error = output.is_error,
            "tool result"
        );

        if output.is_error {
            warn!(
                turn,
                tool = %call.tool_name,
                error = %output.value,
                "tool returned an error"
            );
        }

        info!(
            turn,
            tool = %call.tool_name,
            elapsed_ms = started.elapsed().as_millis() as u64,
            is_error = output.is_error,
            "tool completed"
        );

        self.messages.push(ModelMessage {
            role: Role::Tool,
            content: MessageContent::Parts(vec![ContentPart::ToolResult {
                tool_call_id: call.tool_call_id,
                tool_name: Some(call.tool_name),
                result: output.value,
                is_error: Some(output.is_error),
                preliminary: None,
                dynamic: None,
                provider_options: None,
            }]),
        });

        Ok(())
    }

    fn final_response(text: String, reason: FinishReasonUnified) -> Result<String> {
        if reason != FinishReasonUnified::Stop {
            return Err(ConversationError::IncompleteResponse { reason });
        }

        if text.trim().is_empty() {
            return Err(ConversationError::EmptyResponse);
        }

        Ok(text)
    }

    fn remaining(&self) -> Result<Duration> {
        TIMEOUT
            .checked_sub(self.started.elapsed())
            .filter(|remaining| !remaining.is_zero())
            .ok_or(ConversationError::Timeout)
    }
}
