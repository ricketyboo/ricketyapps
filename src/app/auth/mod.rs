use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String
}

#[server]
pub async fn try_login(credentials: Credentials) -> Result<(), ServerFnError> {
    if credentials.password == "correct" {
        log!("try_login successful login");
        return Ok(())
    }
    log!("try_login failed login");
    // todo: return 401
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