use yew::prelude::*;

#[component]
pub fn PjPermit() -> Html {
    let submitted = use_state(|| false);

    let on_apply = {
        let submitted = submitted.clone();

        Callback::from(move |_| {
            submitted.set(true);
        })
    };

    html! {
        <div>
            if *submitted {
                <div>{"PJ Permit application has been submitted, you will be notified verbally if your application has been accepted."}</div>
            } else {
                <div>
                    <div>{"Press the button to apply for extend duration PJ Permit"}</div>
                    <button onclick={on_apply}>{"Apply"}</button>
                </div>
            }
        </div>
    }
}
