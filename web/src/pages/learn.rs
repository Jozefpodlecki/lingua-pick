use crate::{components::Layout, routes::Route, state::LearningContext};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Learn)]
pub fn learn() -> Html {
    let context = use_context::<LearningContext>();
    let selected = context.as_ref().and_then(|context| context.selected());
    let content = if let Some(language) = selected {
        html! {
            <section>
                <h1 class="mb-4 text-4xl font-semibold">{&language.name}</h1>
                <p class="text-gray-400">{"Your language is selected. Lessons will be added here."}</p>
            </section>
        }
    } else {
        html! {
            <section>
                <h1 class="mb-4 text-3xl font-semibold">{"Choose a language to get started"}</h1>
                <Link<Route> to={Route::Home} classes="text-teal-300 underline">{"Choose a language"}</Link<Route>>
            </section>
        }
    };
    html! {
        <Layout>
            {content}
        </Layout>
    }
}
