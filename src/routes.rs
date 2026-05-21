use yew::{Html, html};
use yew_router::prelude::*;

use crate::app::{Home, NotFound, Tools};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tools")]
    Tools,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn route_switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Tools => html! { <Tools /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
