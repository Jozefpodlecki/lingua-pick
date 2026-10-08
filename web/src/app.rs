use alloc::string::ToString;
use alloc::{rc::Rc, string::String, vec::Vec};
use core::{cell::Cell, str::FromStr};
use lingua_web_core::LoadingScreen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::{HashRouter, Switch};

use crate::client::{ApiClient, ApiError, LoadResult};
use crate::env::AppEnvironment;
use crate::routes::{Route, switch};
use crate::state::LearningContext;

#[function_component(CatalogueBootstrap)]
fn catalogue_bootstrap() -> Html {
    let app = use_context::<AppEnvironment>().unwrap();
    let result = use_state(|| None::<Result<LoadResult, ApiError>>);
    let request_version = use_state(|| 0_u32);
    let client = app.client.clone();

    {
        let result_setter = result.clone();
        let client = client.clone();

        use_effect_with((), move |_| {
            let cancelled = Rc::new(Cell::new(false));
            let request_cancelled = cancelled.clone();

            spawn_local(async move {
                let result = client.load()
                    .await;

                if !request_cancelled.get() {
                    result_setter.set(Some(result));
                }
            });

            move || cancelled.set(true)
        });
    }

    let onretry = {
        let catalogue = result.clone();
        let request_version = request_version.clone();

        Callback::from(move |_| {
            catalogue.set(None);
            request_version.set((*request_version).wrapping_add(1));
        })
    };

    match result.as_ref() {
        None => html! { <LoadingScreen message="Loading languages" /> },
        Some(Ok(result)) => {
            let select_language = {
                let client = client.clone();

                Callback::from(move |id: lingua_core::LanguageId| {
                    client.set_language(&id);
                })
            };

            let learning_context = LearningContext {
                catalogue: result.catalogue.clone(),
                selected_language: result.selected_language_id.clone(),
                select_language,
            };

            html! {
                <ContextProvider<ApiClient> context={client}>
                    <ContextProvider<LearningContext> context={learning_context}>
                        <HashRouter>
                            <Switch<Route> render={switch} />
                        </HashRouter>
                    </ContextProvider<LearningContext>>
                </ContextProvider<ApiClient>>
            }
        },
        Some(Err(error)) => html! {
            <main class="flex min-h-screen items-center justify-center bg-gray-950 px-6 text-gray-100" data-state="error">
                <section class="max-w-lg text-center">
                    <h1 class="mb-3 text-3xl font-semibold">{"Languages could not be loaded"}</h1>
                    <p role="alert" class="mb-6 text-gray-300">{error.to_string()}</p>
                    <button type="button" onclick={onretry}
                        class="rounded-lg bg-teal-700 px-5 py-3 focus-visible:outline-2 focus-visible:outline-teal-400">
                        {"Retry"}
                    </button>
                </section>
            </main>
        },
    }
}

#[function_component(App)]
pub fn app(props: &AppEnvironment) -> Html {
    html! {
        <ContextProvider<AppEnvironment> context={props.clone()}>
            <CatalogueBootstrap />
        </ContextProvider<AppEnvironment>>
    }
}
