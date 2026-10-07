use crate::{
    components::LanguageCard,
    routes::Route,
    state::{LanguageId, LearningContext},
};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

const SUGGESTION_LIMIT: usize = 4;

#[function_component(LanguagePicker)]
pub fn language_picker() -> Html {
    let context = use_context::<LearningContext>();
    let navigator = use_navigator();
    let selected_language = context
        .as_ref()
        .and_then(|context| context.selected_language.clone());
    let query = use_state(String::new);
    let active = use_state(|| None::<usize>);
    let open = use_state(|| true);

    let mut matches: Vec<_> = context
        .as_ref()
        .into_iter()
        .flat_map(|context| context.catalogue.iter())
        .filter(|language| language.matches(&query))
        .collect();
    if !query.trim().is_empty() {
        matches.sort_by_key(|language| (language.search_rank(&query), language.name.as_str()));
    }
    let total = matches.len();
    matches.truncate(SUGGESTION_LIMIT);
    let suggestion_ids: Vec<_> = matches.iter().map(|language| language.id.clone()).collect();
    let oninput = {
        let query = query.clone();
        let active = active.clone();
        let open = open.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                query.set(input.value());
                active.set(None);
                open.set(true);
            }
        })
    };
    let onfocus = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let selection_context = context.clone();
    let onselect = Callback::from(move |id: LanguageId| {
        if let (Some(context), Some(navigator)) = (&selection_context, &navigator) {
            context.select_language.emit(id);
            navigator.push(&Route::Learn);
        }
    });
    let onkeydown = {
        let active = active.clone();
        let open = open.clone();
        let onselect = onselect.clone();
        let ids = suggestion_ids.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.is_composing() {
                return;
            }
            match event.key().as_str() {
                "ArrowDown" | "ArrowUp" if !ids.is_empty() => {
                    event.prevent_default();
                    let next = next_active(*active, ids.len(), event.key() == "ArrowDown");
                    active.set(next);
                    open.set(true);
                }
                "Enter" if *open => {
                    if let Some(id) = active
                        .and_then(|index| ids.get(index))
                        .or_else(|| ids.first())
                    {
                        event.prevent_default();
                        onselect.emit(id.clone());
                    }
                }
                "Escape" => {
                    active.set(None);
                    open.set(false);
                }
                "Tab" => {
                    active.set(None);
                    open.set(false);
                }
                _ => {}
            }
        })
    };
    let active_descendant = (*active)
        .filter(|_| *open)
        .and_then(|index| suggestion_ids.get(index))
        .map(|id| format!("language-option-{}", id.0));
    let active_for_scroll = active_descendant.clone();
    use_effect_with(active_for_scroll, |id| {
        if let Some(element) = id
            .as_ref()
            .and_then(|id| web_sys::window()?.document()?.get_element_by_id(id))
        {
            element.scroll_into_view_with_bool(false);
        }
        || ()
    });
    let empty = (total == 0).to_string();
    let expanded = (*open).to_string();
    let cards = matches.iter().enumerate().map(|(index, language)| {
        let selected = selected_language.as_ref() == Some(&language.id);
        html! {
            <LanguageCard key={language.id.0.clone()} language={(*language).clone()}
                selected={selected} active={*active == Some(index)} onselect={onselect.clone()} />
        }
    }).collect::<Html>();

    html! {
        <section class="group/picker mx-auto max-w-3xl" data-empty={empty} data-open={expanded.clone()}>
            <h1 class="mb-3 text-4xl font-semibold">{"What would you like to learn?"}</h1>
            <input id="language-search" type="text" role="combobox" autocomplete="off"
                aria-label="Search languages" aria-autocomplete="list" aria-haspopup="listbox" aria-expanded={expanded}
                aria-controls="language-results" aria-activedescendant={active_descendant}
                value={(*query).clone()} {oninput} {onfocus} {onkeydown}
                placeholder="Try Portuguese, Brazil, or Japan"
                class="mb-3 w-full rounded-xl border border-gray-700 bg-gray-900 px-4 py-3 focus-visible:outline-2 focus-visible:outline-teal-400" />
            <div class="group-data-[open=false]/picker:hidden">
                <div id="language-results" role="listbox" aria-label="Language suggestions"
                    class="grid gap-4 p-1 sm:grid-cols-2">{cards}</div>
                <p class="hidden py-8 text-gray-400 group-data-[empty=true]/picker:block">{"No matching languages. Try another name or region."}</p>
            </div>
        </section>
    }
}

fn next_active(current: Option<usize>, count: usize, down: bool) -> Option<usize> {
    if count == 0 {
        return None;
    }
    Some(match (current, down) {
        (Some(index), true) => (index + 1) % count,
        (Some(index), false) => (index + count - 1) % count,
        (None, true) => 0,
        (None, false) => count - 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyboard_navigation_wraps_and_handles_empty_results() {
        assert_eq!(next_active(None, 3, true), Some(0));
        assert_eq!(next_active(None, 3, false), Some(2));
        assert_eq!(next_active(Some(2), 3, true), Some(0));
        assert_eq!(next_active(Some(0), 3, false), Some(2));
        assert_eq!(next_active(None, 0, true), None);
    }
}
