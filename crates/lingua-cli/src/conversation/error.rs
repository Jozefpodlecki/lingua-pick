
use lingua_ai::FinishReasonUnified;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversationError {
    #[error("conversation timed out")]
    Timeout,

    #[error("model request failed on turn {turn}: {message}")]
    Model {
        turn: usize,
        message: String,
    },

    #[error("incomplete model response: {reason:?}")]
    IncompleteResponse {
        reason: FinishReasonUnified,
    },

    #[error("model returned an empty response")]
    EmptyResponse,

    #[error("incomplete tool-call turn: {reason:?}")]
    IncompleteToolCalls {
        reason: FinishReasonUnified,
    },

    #[error("model requested tools after they were disabled")]
    ToolsDisabled,

    #[error("tool budget exceeded: requested {requested}, remaining {remaining}")]
    ToolBudgetExceeded {
        requested: usize,
        remaining: usize,
    },

    #[error("tool call ID is empty")]
    EmptyToolCallId,

    #[error("duplicate tool call ID: {id}")]
    DuplicateToolCallId {
        id: String,
    },

    #[error("assistant tool-call message is missing")]
    MissingAssistantMessage,

    #[error("tool execution timed out: {tool}")]
    ToolTimeout {
        tool: String,
    },

    #[error("model turn budget exhausted: {max} turns")]
    TurnBudgetExceeded {
        max: usize,
    },
}
