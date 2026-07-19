use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[component]
pub fn PjPermit() -> Html {
    let submitted = use_state(|| false);
    let name = use_state(String::new);
    let error = use_state(|| None::<String>);

    let on_name_input = {
        let name = name.clone();
        let error = error.clone();

        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();
            name.set(input.value());
            error.set(None);
        })
    };

    let on_apply = {
        let submitted = submitted.clone();
        let name = name.clone();
        let error = error.clone();

        Callback::from(move |_| {
            if name.trim().is_empty() {
                error.set(Some("Please enter your name before applying.".to_string()));
                return;
            }

            error.set(None);
            submitted.set(true);

            spawn_local(async {
                match Request::get(
                    "https://6u74h07g5c.execute-api.us-east-1.amazonaws.com/prod/pj_permit",
                )
                .send()
                .await
                {
                    Ok(_) => tracing::info!("PJ Permit notification sent"),
                    Err(e) => tracing::error!("Failed to send PJ Permit notification: {:?}", e),
                }
            });
        })
    };

    html! {
        <div class="content-container">
            if *submitted {
                <div>{format!("PJ Permit application has been submitted for {}, you will be notified verbally if your application has been accepted.", name.trim())}</div>
                <div class="thankyou">{"Thank you for your application!"}</div>
            } else {
                <div>{"Press the button to apply for extend duration PJ Permit"}</div>
                <div class="user-inputs">
                    <label for="pj-permit-name">{"Name:"}</label>
                    <input
                        id="pj-permit-name"
                        type="text"
                        value={(*name).clone()}
                        oninput={on_name_input}
                        placeholder="Enter your name"
                    />
                    <button onclick={on_apply}>{"Apply"}</button>
                    if let Some(message) = &*error {
                        <div role="alert">{message}</div>
                    }
                </div>
            }
        </div>
    }
}
