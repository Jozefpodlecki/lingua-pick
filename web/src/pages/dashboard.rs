use crate::{
    components::{DashboardCard, Layout},
    routes::Route,
    state::LearningContext,
};
use yew::prelude::*;
use yew_icons::IconData;
use yew_router::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let context = use_context::<LearningContext>();
    let selected = context.as_ref().and_then(|context| context.selected());

    let content = if let Some(language) = selected {
        html! {
            <section>
                <h1 class="mb-8 text-4xl font-semibold">{&language.name}</h1>
                <div class="grid gap-4 sm:grid-cols-3">
                    <DashboardCard label="Run exercise" route={Route::Exercise} icon={IconData::LUCIDE_PLAY} />
                    <DashboardCard label="Analyze stats" route={Route::Stats} icon={IconData::LUCIDE_BAR_CHART_3} />
                    <DashboardCard label="Exercise history" route={Route::History} icon={IconData::LUCIDE_HISTORY} />
                </div>
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

    html! { <Layout>{content}</Layout> }
}
