use yew::prelude::*;

#[component]
pub fn App() -> Html {
    html! {
        <main>
            <h1>{ "black maple software" }</h1>

            <footer>
                <span class="info">{ "made with Rust + Yew + " }<i class="heart" /></span>
            </footer>
        </main>
    }
}
