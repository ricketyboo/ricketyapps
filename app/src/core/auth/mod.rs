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
pub async fn try_login(username:String, password:String) -> Result<(), ServerFnError> {
    log!("try_login {username},{password}");
    Ok(())
}

#[component]
pub fn Login() -> impl IntoView {
    let submit = ServerAction::<TryLogin>::new();
    let validate = move |event: SubmitEvent| {
        let data = Credentials::from_event(&event);
        log!("{:?}", data);
        match data {
            Ok(credentials) => {
                if credentials.username.is_empty() || credentials.password.is_empty() {
                    log!("Invalid data");
                    event.prevent_default();
                }
                // else {
                //     submit.dispatch(TryLogin::from(credentials));
                // }
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
        <ActionForm action=submit on:submit:capture=validate>
            <label>"username"<input name="username" /></label>
            <label>"password"<input name="password" type="password" /></label>
            <button>Login</button>
        </ActionForm>
    }
}
