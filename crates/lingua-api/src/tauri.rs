use alloc::{
    format,
    string::{String, ToString},
};
use core::fmt;
use lingua_ai::{PromptError, PromptRequest, PromptResponse};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::Runtime;

const SEND_PROMPT_COMMAND: &str = "send_prompt";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        catch,
        js_namespace = ["window", "__TAURI__", "core"],
        js_name = invoke
    )]
    async fn invoke(command: &str, arguments: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct SendPromptArguments<'a> {
    request: &'a PromptRequest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TauriPromptError {
    UnsupportedRuntime,
    Arguments(String),
    Command(PromptError),
    Bridge(String),
    Response(String),
}

impl fmt::Display for TauriPromptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedRuntime => {
                formatter.write_str("AI generation requires the desktop application.")
            }
            Self::Arguments(details) => {
                write!(formatter, "Could not prepare AI request: {details}")
            }
            Self::Command(error) => error.fmt(formatter),
            Self::Bridge(details) => write!(formatter, "Desktop communication failed: {details}"),
            Self::Response(details) => write!(formatter, "Invalid desktop response: {details}"),
        }
    }
}

impl core::error::Error for TauriPromptError {}

pub async fn send_prompt(
    runtime: Runtime,
    request: &PromptRequest,
) -> Result<PromptResponse, TauriPromptError> {
    if runtime != Runtime::Tauri {
        return Err(TauriPromptError::UnsupportedRuntime);
    }

    let arguments = serde_wasm_bindgen::to_value(&SendPromptArguments { request })
        .map_err(|error| TauriPromptError::Arguments(error.to_string()))?;
    let response = invoke(SEND_PROMPT_COMMAND, arguments)
        .await
        .map_err(parse_invoke_error)?;

    serde_wasm_bindgen::from_value(response)
        .map_err(|error| TauriPromptError::Response(error.to_string()))
}

fn parse_invoke_error(value: JsValue) -> TauriPromptError {
    serde_wasm_bindgen::from_value::<PromptError>(value.clone())
        .map(TauriPromptError::Command)
        .unwrap_or_else(|_| {
            let message = value.as_string().unwrap_or_else(|| format!("{value:?}"));

            TauriPromptError::Bridge(message)
        })
}
