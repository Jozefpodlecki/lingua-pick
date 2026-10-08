use crate::{components::Layout, routes::Route, state::LearningContext};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ActivityProps {
    pub history: bool,
}

#[function_component(Activity)]
pub fn activity(props: &ActivityProps) -> Html {
    let context = use_context::<LearningContext>();
    let selected = context.as_ref().and_then(|context| context.selected());
    let title = if props.history {
        "Exercise history"
    } else {
        "Analyze stats"
    };
    let message = if props.history {
        "Exercise history is not saved yet."
    } else {
        "Statistics are not recorded yet."
    };
    let destination = if selected.is_some() {
        Route::Learn
    } else {
        Route::Home
    };
    let back_label = if selected.is_some() {
        "Back"
    } else {
        "Choose a language"
    };
    let language_name = selected.map(|language| language.name.clone());

    html! {
        <Layout>
            <section>
                <Link<Route> to={destination} classes="mb-6 inline-block text-teal-300 underline">{back_label}</Link<Route>>
                <h1 class="mb-4 text-3xl font-semibold">{title}</h1>
                <p class="mb-4 text-gray-300">{language_name}</p>
                <p class="text-gray-400">{message}</p>
            </section>
        </Layout>
    }
}
