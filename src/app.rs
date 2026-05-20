use yew::prelude::*;

#[component]
pub fn App() -> Html {
    html! {
        <main>
            <img width="150" src="assets/36_maple.svg" alt="Black Maple Logo" />
            <h1>{ "black maple" }<br/>{ "software" }</h1>

            <footer>
                <span class="info">{ "made with Rust + Yew + " }<i class="heart" /></span>
            </footer>
        </main>
    }
}
