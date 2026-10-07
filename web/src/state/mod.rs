mod language;

pub use language::{Language, LanguageId, load_catalogue};

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LearningContext {
    pub catalogue: alloc::rc::Rc<alloc::vec::Vec<Language>>,
    pub selected_language: Option<LanguageId>,
    pub select_language: Callback<LanguageId>,
    pub storage_warning: Option<alloc::string::String>,
}

impl LearningContext {
    pub fn selected(&self) -> Option<&Language> {
        let id = self.selected_language.as_ref()?;
        self.catalogue.iter().find(|language| &language.id == id)
    }
}
