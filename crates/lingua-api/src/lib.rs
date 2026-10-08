#![no_std]

extern crate alloc;

mod runtime;
mod tauri;

pub use runtime::{Runtime, RuntimeError};
pub use tauri::{TauriPromptError, send_prompt};
