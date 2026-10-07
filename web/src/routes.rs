use yew::prelude::*;
use yew_router::Routable;

use crate::pages::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/learn")]
    Learn,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Learn => html! { <Learn /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
