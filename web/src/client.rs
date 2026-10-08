use core::fmt;

use alloc::{format, string::String};
use gloo::net::http::Request;
use lingua_core::{Language, LanguageId, Languages};
use web_sys::Storage;

use crate::services::storage::StorageService;

const SELECTED_LANGUAGE_KEY: &str = "lingua-pick.selected-language.v1";
const CATALOGUE_PATH: &str = "languages.json";

#[derive(Debug, Clone, PartialEq)]
pub struct ApiClient {
    selected_lang: StorageService,
}

#[derive(Debug)]
pub enum ApiError {
    Request(String),
    Http(u16),
    Storage(String),
    Serde(String),
    Invalid(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(details) => {
                write!(formatter, "Could not request the language catalogue: {details}")
            }
            Self::Http(status) => {
                write!(
                    formatter,
                    "Could not load the language catalogue (HTTP {status})."
                )
            }
            Self::Storage(details) => {
                write!(formatter, "Could not access application storage: {details}")
            }
            Self::Serde(details) => {
                write!(formatter, "Could not deserialize response: {details}")
            }
            Self::Invalid(details) => {
                write!(formatter, "Invalid language catalogue: {details}")
            }
        }
    }
}

impl core::error::Error for ApiError {}

#[derive(Debug)]
pub struct LoadResult {
    pub catalogue: Languages,
    pub selected_language_id: Option<LanguageId>,
    pub selected_language: Option<Language>,
}

impl ApiClient {
    pub fn new(storage: Storage) -> Self {
        Self {
            selected_lang: StorageService::new(
                storage,
                SELECTED_LANGUAGE_KEY,
            ),
        }
    }

    pub async fn load(&self) -> Result<LoadResult, ApiError> {
        let catalogue = self.get_catalogue().await?;
        let selected_language_id = self.get_current_language();

        let selected_language = selected_language_id
            .as_ref()
            .and_then(|id| {
                catalogue
                    .iter()
                    .find(|language| &language.id == id)
                    .cloned()
            });

        Ok(LoadResult {
            catalogue,
            selected_language_id,
            selected_language,
        })
    }

    pub fn set_language(&self, id: &LanguageId) -> Result<(), ApiError> {
        self.selected_lang
            .save(&id)
            .map_err(|error| ApiError::Storage(format!("{error}")))
    }

    pub fn get_current_language(&self) -> Option<LanguageId> {
        self.selected_lang
            .load::<LanguageId>()
    }

    pub async fn get_catalogue(&self) -> Result<Languages, ApiError> {
        let response = Request::get(CATALOGUE_PATH)
            .send()
            .await
            .map_err(|error| ApiError::Request(format!("{error}")))?;

        if !response.ok() {
            return Err(ApiError::Http(response.status()));
        }

        let data = response
            .json()
            .await
            .map_err(|error| ApiError::Serde(format!("{error}")))?;

        Ok(Languages::new(data))
    }
}