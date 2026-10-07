use crate::components::{LanguagePicker, Layout};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Layout>
            <LanguagePicker />
        </Layout>
    }
}
