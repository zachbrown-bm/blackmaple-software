use yew::{Html, html};
use yew_router::prelude::*;

use crate::app::{HomePage, NotFound, PjPermitPage, ToolsPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tools")]
    Tools,
    #[at("/pjpermit")]
    PjPermit,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn route_switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Tools => html! { <ToolsPage /> },
        Route::PjPermit => html! { <PjPermitPage /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
