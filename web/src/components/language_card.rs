use alloc::string::ToString;
use lingua_core::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LanguageCardProps {
    pub language: Language,
    pub selected: bool,
    pub active: bool,
    pub onselect: Callback<LanguageId>,
}

#[function_component(LanguageCard)]
pub fn language_card(props: &LanguageCardProps) -> Html {
    let id = props.language.id.clone();
    let onselect = props.onselect.clone();
    let onclick = Callback::from(move |_| onselect.emit(id.clone()));
    let selected = props.selected.to_string();
    let active = props.active.to_string();
    let option_id = alloc::format!("language-option-{}", props.language.id.0);
    html! {
        <div id={option_id} role="option" {onclick} data-language-id={props.language.id.0.clone()}
            data-selected={selected} data-active={active.clone()} aria-selected={active}
            class="group cursor-pointer rounded-2xl border border-gray-700 bg-gray-900 p-6 text-left transition-colors hover:border-teal-400 data-[selected=true]:border-teal-400 data-[active=true]:bg-teal-950 data-[active=true]:border-teal-300">
            <span class="mb-3 block text-sm text-teal-300">{&props.language.region}</span>
            <span class="block text-xl font-semibold">{&props.language.name}</span>
            <span class="mt-2 block text-sm text-gray-400">{&props.language.native_name}</span>
            <span class="mt-4 hidden text-sm text-teal-300 group-data-[selected=true]:block">{"Selected · Continue"}</span>
        </div>
    }
}
