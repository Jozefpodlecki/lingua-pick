use crate::{components::TopBar, state::LearningContext};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let context = use_context::<LearningContext>();
    let warning = context.and_then(|context| context.storage_warning);
    let warning_content = warning.map(|message| html! {
        <p role="status" class="mb-6 rounded-lg border border-amber-700 p-3 text-sm text-amber-200">{message}</p>
    }).unwrap_or_default();
    html! {
        <div class="min-h-screen bg-gray-950 text-gray-100">
            <TopBar />
            <main class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
                {warning_content}
                { for props.children.iter() }
            </main>
        </div>
    }
}
