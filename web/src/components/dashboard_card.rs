use crate::routes::Route;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DashboardCardProps {
    pub label: AttrValue,
    pub route: Route,
    pub icon: IconData,
}

#[function_component(DashboardCard)]
pub fn dashboard_card(props: &DashboardCardProps) -> Html {
    html! {
        <Link<Route> to={props.route.clone()}
            classes="flex min-h-40 flex-col items-start justify-between gap-6 rounded-2xl border border-gray-700 bg-gray-900 p-6 transition-colors hover:border-teal-400 focus-visible:outline-2 focus-visible:outline-teal-400">
            <span aria-hidden="true" class="text-teal-300">
                <Icon data={props.icon} width="28px" height="28px" />
            </span>
            <span class="text-xl font-semibold">{&props.label}</span>
        </Link<Route>>
    }
}
