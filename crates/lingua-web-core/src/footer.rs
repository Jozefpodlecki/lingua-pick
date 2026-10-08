use yew::prelude::*;
use yew_icons::{Icon, IconData};

const REPOSITORY_URL: &str = "https://github.com/Jozefpodlecki/lingua-pick";

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub runtime_name: AttrValue,
    pub runtime_label: AttrValue,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    html! {
        <footer class="border-t border-gray-800" data-runtime={props.runtime_name.clone()}>
            <div class="mx-auto flex w-full max-w-6xl flex-col gap-3 px-4 py-5 text-sm text-gray-400 sm:flex-row sm:items-center sm:justify-between sm:px-6 lg:px-8">
                <span>{"Jozef Podlecki © 2026"}</span>
                <a href={REPOSITORY_URL} target="_blank" rel="noopener noreferrer"
                    class="inline-flex w-fit items-center gap-2 text-gray-300 hover:text-teal-300 focus-visible:outline-2 focus-visible:outline-teal-400">
                    <span aria-hidden="true">
                        <Icon data={IconData::LUCIDE_GITHUB} width="18px" height="18px" />
                    </span>
                </a>
                <span class="w-fit rounded-full border border-gray-700 px-3 py-1 text-xs text-gray-300">
                    {"Mode: "}{&props.runtime_label}
                </span>
            </div>
        </footer>
    }
}
