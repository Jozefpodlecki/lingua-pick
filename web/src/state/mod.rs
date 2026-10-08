
use lingua_core::{Language, LanguageId, language::Languages};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LearningContext {
    pub catalogue: Languages,
    pub selected_language: Option<LanguageId>,
    pub select_language: Callback<LanguageId>
}

impl LearningContext {
    pub fn selected(&self) -> Option<&Language> {
        let id = self.selected_language.as_ref()?;
        self.catalogue.iter().find(|language| &language.id == id)
    }
}
