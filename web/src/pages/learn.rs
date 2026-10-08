use crate::{components::LearningSession, routes::Route, state::LearningContext};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Learn)]
pub fn learn() -> Html {
    let context = use_context::<LearningContext>().unwrap();
    let selected = context.selected();

    html! {

    }
    // if let (Some(language), Some(app)) = (selected, {
    //     html! {
    //         <LearningSession key={language.id.0.clone()} />
    //     }
    // } else {
    //     html! {
    //         <main class="flex min-h-screen items-center justify-center bg-gray-950 px-4 text-gray-100">
    //             <section class="text-center">
    //                 <h1 class="mb-4 text-3xl font-semibold">{"Choose a language to get started"}</h1>
    //                 <Link<Route> to={Route::Home} classes="text-teal-300 underline">{"Choose a language"}</Link<Route>>
    //             </section>
    //         </main>
    //     }
    // }
}
