use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub(crate) mod ssr;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String
}

#[server]
pub async fn try_login(credentials: Credentials) -> Result<(), ServerFnError> {
    log!("try_login {:?}", credentials);
    if credentials.password == "correct" {
        return Ok(())
    }
    Err(ServerFnError::Response("Invalid credentials".into()))
}

#[component]
pub fn Login() -> impl IntoView {
    let submit_action = ServerAction::<TryLogin>::new();
    // todo: how to get this? want to respond to the error. Maybe this isn't possible client side with ActionForm
    //  https://docs.rs/reactive_graph/0.2.2/reactive_graph/actions/struct.Action.html
    // let v = submit_action.value();
    let validate = move |event: SubmitEvent| {
        // this is kind of making the ActionForm action redundant; but if I don't manual dispatch I can't get the credentials to unwrap properly?
        event.prevent_default();
        let data = Credentials::from_event(&event);
        log!("{:?}", data);
        match data {
            Ok(credentials) => {
                if credentials.username.is_empty() || credentials.password.is_empty() {
                    log!("Invalid data");                    
                }
                else {                   
                    submit_action.dispatch(TryLogin::from(credentials));
                }
            }
            Err(..) => {
                // can check deserialisation errors here
                log!("Error with data");
                event.prevent_default();
            }
        }
    };
    view! {
        <h2>"Login"</h2>
        <ActionForm action=submit_action on:submit:capture=validate>
            <label>"username"<input name="username" /></label>
            <label>"password"<input name="password" type="password" /></label>
            <button>Login</button>
        </ActionForm>
    }
}
