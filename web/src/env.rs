use alloc::{rc::Rc, string::String, vec::Vec};
use core::{cell::Cell, str::FromStr};
use lingua_api::{Runtime, RuntimeError};
use log::Level;
use thiserror::Error;
use web_sys::window;
use web_sys::{Document, HtmlElement, Navigator, Storage, Window};
use yew::prelude::*;

use crate::client::ApiClient;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppEnvironment {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub local_storage: Storage,
    pub navigator: Navigator,
    pub app_name: Rc<str>,
    pub version: Rc<str>,
    pub runtime: Runtime,
    pub client: ApiClient
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AppEnvironmentError {
    #[error("Window object not found")]
    NoWindow,

    #[error("localStorage is not available")]
    NoStorage,

    #[error("Failed to access localStorage: {0}")]
    StorageAccess(String),

    #[error("Document object not found")]
    NoDocument,

    #[error("Document body not found")]
    NoBody,

    #[error("Could not detect application runtime: {0}")]
    Runtime(#[source] RuntimeError),
}

impl AppEnvironment {
    pub fn new() -> Result<Self, AppEnvironmentError> {
        let window = window().ok_or(AppEnvironmentError::NoWindow)?;
        let runtime = Runtime::detect(&window).map_err(AppEnvironmentError::Runtime)?;

        let local_storage = window
            .local_storage()
            .map_err(|error| AppEnvironmentError::StorageAccess(alloc::format!("{error:?}")))?
            .ok_or(AppEnvironmentError::NoStorage)?;

        let document = window.document().ok_or(AppEnvironmentError::NoDocument)?;
        let body = document.body().ok_or(AppEnvironmentError::NoBody)?;
        let navigator = window.navigator();
        let app_name = env!("CARGO_PKG_NAME").into();
        let version = env!("CARGO_PKG_VERSION").into();
        let client = ApiClient::new(local_storage.clone());

        Ok(Self {
            window,
            document,
            body,
            local_storage,
            navigator,
            app_name,
            version,
            runtime,
            client
        })
    }

    pub fn try_get_log_level_from_local_storage(&self, key: &str) -> Option<Level> {
        let level_str = self.local_storage.get_item(key).ok().flatten()?;

        Level::from_str(&level_str).ok()
    }
}
