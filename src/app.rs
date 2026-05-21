use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::splash_logo::SplashLogo;
use crate::routes::{Route, route_switch};

#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route_switch}/>
        </BrowserRouter>
    }
}

#[component]
pub fn Home() -> Html {
    html! {
        <main>
            <SplashLogo />
            <h1>{ "black maple" }<br/>{ "software" }</h1>
            <Footer />
        </main>
    }
}

#[component]
pub fn Tools() -> Html {
    html! {
        <main>
            <SplashLogo />
            <h1>{ "black maple" }<br/>{ "software" }</h1>
            <ul>
                <li>{ "GUID Gen" }</li>
                <li>{ "URL Shortener" }</li>
            </ul>
            <Footer />
        </main>
    }
}

#[component]
pub fn NotFound() -> Html {
    html! {
        <main>
            <SplashLogo />
            <h1>{ "Quoth the server, 404" }</h1>
            <Footer />
        </main>
    }
}
