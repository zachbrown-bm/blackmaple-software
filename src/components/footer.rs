use yew::prelude::*;

#[component]
pub fn Footer() -> Html {
    html! {
        <footer>
            <span class="info">{ "made with Rust + Yew + " }<i class="heart" /></span>
        </footer>
    }
}
