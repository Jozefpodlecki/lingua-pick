use alloc::string::String;
use core::fmt;
use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runtime {
    Tauri,
    LocalWeb,
    GitHubPages,
    HostedWeb,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    Inspection {
        field: &'static str,
        details: String,
    },
    MissingCheck,
    CheckFailed(String),
    InvalidCheckResult,
    HostnameAccess(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inspection { field, details } => write!(
                formatter,
                "Could not inspect runtime field {field}: {details}"
            ),
            Self::MissingCheck => formatter.write_str("Desktop runtime check is unavailable."),
            Self::CheckFailed(details) => {
                write!(formatter, "Desktop runtime check failed: {details}")
            }
            Self::InvalidCheckResult => {
                formatter.write_str("Desktop runtime check returned an invalid result.")
            }
            Self::HostnameAccess(details) => {
                write!(formatter, "Could not read app hostname: {details}")
            }
        }
    }
}

impl core::error::Error for RuntimeError {}

impl Runtime {
    pub fn detect(window: &Window) -> Result<Self, RuntimeError> {
        let tauri = Reflect::get(window, &JsValue::from_str("__TAURI__")).map_err(|error| {
            RuntimeError::Inspection {
                field: "__TAURI__",
                details: alloc::format!("{error:?}"),
            }
        })?;

        let is_tauri = if tauri.is_object() {
            let core = Reflect::get(&tauri, &JsValue::from_str("core")).map_err(|error| {
                RuntimeError::Inspection {
                    field: "core",
                    details: alloc::format!("{error:?}"),
                }
            })?;
            let check = Reflect::get(&core, &JsValue::from_str("isTauri"))
                .map_err(|error| RuntimeError::Inspection {
                    field: "isTauri",
                    details: alloc::format!("{error:?}"),
                })?
                .dyn_into::<Function>()
                .map_err(|_| RuntimeError::MissingCheck)?;

            check
                .call0(&core)
                .map_err(|error| RuntimeError::CheckFailed(alloc::format!("{error:?}")))?
                .as_bool()
                .ok_or(RuntimeError::InvalidCheckResult)?
        } else {
            false
        };

        let hostname = window
            .location()
            .hostname()
            .map_err(|error| RuntimeError::HostnameAccess(alloc::format!("{error:?}")))?;

        Ok(Self::classify(is_tauri, &hostname))
    }

    pub fn classify(is_tauri: bool, hostname: &str) -> Self {
        if is_tauri {
            return Self::Tauri;
        }

        let hostname = hostname.trim_end_matches('.').to_lowercase();

        if hostname == "localhost"
            || hostname.ends_with(".localhost")
            || hostname == "127.0.0.1"
            || hostname == "[::1]"
            || hostname == "::1"
        {
            return Self::LocalWeb;
        }

        if hostname.ends_with(".github.io") {
            return Self::GitHubPages;
        }

        Self::HostedWeb
    }

    pub fn uses_samples(self) -> bool {
        self != Self::Tauri
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Tauri => "tauri",
            Self::LocalWeb => "local-web",
            Self::GitHubPages => "github-pages",
            Self::HostedWeb => "hosted-web",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Tauri => "Tauri desktop",
            Self::LocalWeb => "Local web",
            Self::GitHubPages => "GitHub Pages",
            Self::HostedWeb => "Hosted web",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_takes_precedence_and_web_modes_are_sample_only() {
        assert_eq!(Runtime::classify(true, "localhost"), Runtime::Tauri);
        assert_eq!(Runtime::classify(true, "tauri.localhost"), Runtime::Tauri);
        assert!(!Runtime::Tauri.uses_samples());

        for hostname in [
            "localhost",
            "localhost.",
            "127.0.0.1",
            "[::1]",
            "app.localhost",
        ] {
            let runtime = Runtime::classify(false, hostname);
            assert_eq!(runtime, Runtime::LocalWeb);
            assert!(runtime.uses_samples());
        }

        assert_eq!(
            Runtime::classify(false, "jozefpodlecki.github.io"),
            Runtime::GitHubPages
        );
        assert_eq!(
            Runtime::classify(false, "github.io.evil.example"),
            Runtime::HostedWeb
        );
        assert!(Runtime::GitHubPages.uses_samples());
        assert!(Runtime::HostedWeb.uses_samples());
    }
}
