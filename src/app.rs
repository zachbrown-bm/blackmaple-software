use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::splash_logo::SplashLogo;

#[component]
pub fn App() -> Html {
    html! {
        <main>
            <SplashLogo />
            <h1>{ "black maple" }<br/>{ "software" }</h1>

            <Footer />
        </main>
    }
}
