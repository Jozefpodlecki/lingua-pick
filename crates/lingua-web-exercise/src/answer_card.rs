use alloc::string::{String, ToString};
use lingua_core::Choice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AnswerCardProps {
    pub choice: Choice,
    pub selected: bool,
    pub onselect: Callback<String>,
}

#[function_component(AnswerCard)]
pub fn answer_card(props: &AnswerCardProps) -> Html {
    let id = props.choice.id.clone();
    let onselect = props.onselect.clone();
    let onclick = Callback::from(move |_| onselect.emit(id.clone()));
    let selected = props.selected.to_string();

    html! {
        <button type="button" {onclick} data-selected={selected.clone()} aria-pressed={selected}
            class="rounded-xl border border-gray-700 p-5 text-left data-[selected=true]:border-teal-400 focus-visible:outline-2 focus-visible:outline-teal-400">
            {&props.choice.text.value}
        </button>
    }
}
