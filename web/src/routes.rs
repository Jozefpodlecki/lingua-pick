use yew::prelude::*;
use yew_router::Routable;

use crate::pages::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/learn")]
    Learn,
    #[at("/learn/exercise")]
    Exercise,
    #[at("/learn/stats")]
    Stats,
    #[at("/learn/history")]
    History,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Learn => html! { <Dashboard /> },
        Route::Exercise => html! { <Learn /> },
        Route::Stats => html! { <Activity history={false} /> },
        Route::History => html! { <Activity history={true} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
