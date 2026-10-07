use std::rc::Rc;

use alloc::string::String;
use core::str::FromStr;
use log::Level;
use thiserror::Error;
use web_sys::window;
use web_sys::{Document, HtmlElement, Navigator, Storage, Window};
use yew::prelude::*;
use yew_router::{HashRouter, Switch};

use crate::routes::{Route, switch};
use crate::state::{Language, LanguageId, LearningContext, load_catalogue};
use crate::storage::StorageService;

const SELECTED_LANGUAGE_KEY: &str = "lingua-pick.selected-language.v1";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppContext {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub local_storage: Storage,
    pub catalogue: Rc<alloc::vec::Vec<Language>>,
    pub navigator: Navigator,
    pub app_name: Rc<str>,
    pub version: Rc<str>,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AppContextError {
    #[error("Window object not found")]
    NoWindow,

    #[error("localStorage is not available")]
    NoStorage,

    #[error("Invalid language catalogue: {0}")]
    Catalogue(String),

    #[error("Failed to access localStorage: {0}")]
    StorageAccess(String),

    #[error("Document object not found")]
    NoDocument,

    #[error("Document body not found")]
    NoBody,
}

impl AppContext {
    pub fn new() -> Result<Self, AppContextError> {
        let window = window().ok_or(AppContextError::NoWindow)?;

        let local_storage = window
            .local_storage()
            .map_err(|error| AppContextError::StorageAccess(alloc::format!("{error:?}")))?
            .ok_or(AppContextError::NoStorage)?;

        let document = window.document().ok_or(AppContextError::NoDocument)?;

        let body = document.body().ok_or(AppContextError::NoBody)?;

        let navigator = window.navigator();
        let catalogue = Rc::new(
            load_catalogue(include_str!("../../assets/languages.json"))
                .map_err(AppContextError::Catalogue)?,
        );
        let app_name = env!("CARGO_PKG_NAME").into();
        let version = env!("CARGO_PKG_VERSION").into();

        Ok(Self {
            window,
            document,
            body,
            local_storage,
            catalogue,
            navigator,
            app_name,
            version,
        })
    }

    pub fn try_get_log_level_from_local_storage(&self, key: &str) -> Option<Level> {
        let level_str = self.local_storage.get_item(key).ok().flatten()?;

        Level::from_str(&level_str).ok()
    }
}

#[function_component(App)]
pub fn app(props: &AppContext) -> Html {
    let storage = props.local_storage.clone();
    let catalogue = props.catalogue.clone();
    let selected_language = use_state(move || {
        StorageService::new(storage, SELECTED_LANGUAGE_KEY)
            .load::<LanguageId>()
            .filter(|id| catalogue.iter().any(|language| &language.id == id))
    });
    let storage_warning = use_state(|| None::<String>);
    let select_language =
        {
            let selected_language = selected_language.clone();
            let storage_warning = storage_warning.clone();
            let storage = props.local_storage.clone();
            Callback::from(move |language: LanguageId| {
                selected_language.set(Some(language.clone()));
                let result =
                    StorageService::new(storage.clone(), SELECTED_LANGUAGE_KEY).save(&language);
                storage_warning.set(result.err().map(|_| String::from(
                "Your language changed, but could not be saved. It may reset when you reload."
            )));
            })
        };
    let learning_context = LearningContext {
        catalogue: props.catalogue.clone(),
        selected_language: (*selected_language).clone(),
        select_language,
        storage_warning: (*storage_warning).clone(),
    };

    html! {
        <ContextProvider<AppContext> context={props.clone()}>
            <ContextProvider<LearningContext> context={learning_context}>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
            </ContextProvider<LearningContext>>
        </ContextProvider<AppContext>>
    }
}
