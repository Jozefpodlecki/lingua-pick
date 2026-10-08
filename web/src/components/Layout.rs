use crate::{components::TopBar, env::AppEnvironment, state::LearningContext};
use lingua_web_core::Footer;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let context = use_context::<LearningContext>();
    let app = use_context::<AppEnvironment>();
    let runtime = app.as_ref().map(|app| app.runtime);
    let runtime_name = runtime.map_or("unknown", |runtime| runtime.name());
    let runtime_label = runtime.map_or("Unknown", |runtime| runtime.label());

    html! {
        <div class="flex min-h-screen flex-col bg-gray-950 text-gray-100" data-runtime={runtime_name}>
            <TopBar />
            <main class="mx-auto w-full max-w-6xl flex-1 px-4 py-8 sm:px-6 lg:px-8">
                { for props.children.iter() }
            </main>
            <Footer {runtime_name} {runtime_label} />
        </div>
    }
}
