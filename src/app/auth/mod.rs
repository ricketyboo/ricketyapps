use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
mod user;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: Uuid,
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String
}

#[server]
pub async fn try_login(credentials: Credentials) -> Result<(), ServerFnError> {
    use crate::app::auth::user::UserRow;
    use axum::http::StatusCode;
    use crate::db::get_pool;

    // todo: move into main and pass via state/context
    let pool = get_pool().await.ok().unwrap();
    
    if UserRow::get_by_credentials(credentials, &pool).await.is_some() {
        log!("try_login successful login");
        // todo: add support for navigating back to an intended url pre login.
        //  would have to have stored it in session during original auth check
        // todo: set cookie
        leptos_axum::redirect("/");

        return Ok(())
    }
    
    log!("try_login failed login");
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    opts.set_status(StatusCode::UNAUTHORIZED);
    Err(ServerFnError::ServerError("Invalid credentials".into()))
}

#[component]
pub fn Login() -> impl IntoView {
    let submit_action = ServerAction::<TryLogin>::new();
    // todo: how to get this? want to respond to the error. Maybe this isn't possible client side with ActionForm?
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
            <A href="/register">Register</A>
        </ActionForm>
    }
}

#[server]
pub async fn try_register(credentials: Credentials) -> Result<User, ServerFnError> {
    use crate::app::auth::user::UserRow;
    use axum::http::StatusCode;
    use crate::db::get_pool;
    
    // todo: move into main and pass via state/context
    let pool = get_pool().await.ok().unwrap();
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    if let Some(user_row) = UserRow::create(credentials, &pool).await {
        opts.set_status(StatusCode::CREATED);
        // todo: initialise session
        leptos_axum::redirect("/login");
        return Ok(User::from(user_row))
    }
    opts.set_status(StatusCode::BAD_REQUEST);
    Err(ServerFnError::ServerError("Unable to register".into()))
}
#[component]
pub fn Register() -> impl IntoView {
    let submit_action = ServerAction::<TryRegister>::new();
    view! {
        <h2>"Register"</h2>
        <ActionForm action=submit_action>
            <label>"username"<input name="credentials[username]" /></label>
            <label>"password"<input name="credentials[password]" type="password" /></label>
            <button>Login</button>
        </ActionForm>
    }
}

#[cfg(feature = "ssr")]
pub mod utils {
    use argon2::password_hash::SaltString;
    use argon2::Algorithm::Argon2id;
    use argon2::{Argon2, Params, PasswordHasher, Version};
    use rand::thread_rng;
    // todo: proper error returns
    pub async fn hash_password(password: &str) -> Result<String, &'static str> {
        let salt = SaltString::generate(&mut thread_rng());
        let password_hash = Argon2::new(
            Argon2id,
            Version::V0x13,
            // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
            Params::new(19456, 2, 1, None).unwrap(),
        )
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Ok(password_hash)
    } 
}