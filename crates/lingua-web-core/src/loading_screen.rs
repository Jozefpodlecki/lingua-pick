use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingScreenProps {
    #[prop_or(AttrValue::from("Loading"))]
    pub message: AttrValue,
}

#[function_component(LoadingScreen)]
pub fn loading_screen(props: &LoadingScreenProps) -> Html {
    html! {
        <main class="loader-wrapper bg-gray-950 text-gray-100" data-state="loading">
            <div class="loader" aria-hidden="true">
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
                <span class="square"></span>
            </div>
            <p class="sr-only" role="status">{&props.message}</p>
        </main>
    }
}
