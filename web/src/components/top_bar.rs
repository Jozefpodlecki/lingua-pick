use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::Route, state::LearningContext};

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    let context = use_context::<LearningContext>();
    let selected = context.as_ref().and_then(|context| context.selected());
    let language_name = selected.map_or("Choose a language", |language| language.name.as_str());
    let action_label = if selected.is_some() {
        "Change"
    } else {
        "Select language"
    };

    html! {
        <header class="border-b border-gray-800">
            <nav aria-label="Main navigation" class="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-4 px-4 py-5 sm:px-6 lg:px-8">
                <Link<Route> to={Route::Home} classes="text-xl font-semibold focus-visible:outline-2 focus-visible:outline-teal-400">
                    {"Lingua Pick"}
                </Link<Route>>
                <div class="flex flex-wrap items-center gap-3">
                    <span class="text-sm text-gray-300" aria-live="polite">
                        {language_name}
                    </span>
                    <Link<Route> to={Route::Home} classes="rounded-lg border border-gray-700 px-3 py-2 text-sm hover:border-teal-400 focus-visible:outline-2 focus-visible:outline-teal-400">
                        {action_label}
                    </Link<Route>>
                </div>
            </nav>
        </header>
    }
}
