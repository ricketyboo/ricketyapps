use crate::dto::Credentials;
use leptos::logging::log;
use leptos::prelude::*;

use leptos_router::components::A;

#[component]
pub fn LoginPage() -> impl IntoView {
    let action = ServerAction::<TryLogin>::new();
    let value = action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));
    Effect::new(move || {
        if has_error() {
            log!("Auth error {:?}", value.get())
        }
    });
    view! {
        <h2>"Login"</h2>
        <ActionForm action>
            <Show when=move || has_error()>
                <div id="login-error" class="form-error-panel">
                    <p>Unable to login</p>
                </div>
            </Show>
            <label>"username"<input name="credentials[username]" /></label>
            <label>"password"<input name="credentials[password]" type="password" /></label>
            <button>Login</button>
            <footer>
                <p>
                    <small>"Don't have an account? " <A href="../register">"Register"</A></small>
                </p>
                <p>
                    <small>Go Home? <A href="/">Home</A></small>
                </p>
            </footer>
        </ActionForm>
    }
}

#[server(endpoint = "auth/login")]
pub async fn try_login(credentials: Credentials) -> Result<String, ServerFnError> {
    use crate::entities::{User, UserDbError};
    use axum::http::StatusCode;

    let client = common::db::use_client().ok_or_else(|| ServerFnError::new("Server error"))?;

    match User::get_by_credentials(credentials, &client).await {
        Ok(Some(u)) => {
            let auth = crate::session::get_auth_session().await?;

            auth.login_user(u.id);

            // todo: add support for navigating back to an intended url pre login.
            //  would have to have stored it in session during original auth check

            leptos_axum::redirect("/");
            // todo: don't use string value here, it's just a hack to deal with error boundaries in the UI
            Ok("Ok".into())
        }
        Ok(None) => {
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Invalid credentials"))
        }
        Err(e) => match e {
            UserDbError::UsernameExists => {
                // todo: this is a good example of why I should split the errors up, this seems silly to have to deal with
                unreachable!("Username exists error when trying to login")
            }
            UserDbError::UsernameNotExists => {
                let opts = expect_context::<leptos_axum::ResponseOptions>();
                opts.set_status(StatusCode::UNAUTHORIZED);
                Err(ServerFnError::new("Invalid credentials"))
            }
            UserDbError::UnknownError => Err(ServerFnError::new("System error")),
        },
    }
}
