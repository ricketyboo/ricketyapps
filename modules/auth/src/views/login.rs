use crate::dto::Credentials;
use crate::views::login::server_fn::codec::JsonEncoding;
use leptos::logging::log;
use leptos::prelude::*;

use leptos_router::components::A;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AuthenticationError {
    #[error("Something went wrong")]
    ServerFnError(ServerFnErrorErr),
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Something went wrong")]
    InternalServerError,
}
impl FromServerFnError for AuthenticationError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AuthenticationError::ServerFnError(value)
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let registration_available = OnceResource::new_blocking(get_registration_available());
    let action = ServerAction::<TryLogin>::new();
    let value = action.value();
    // let has_error = move || value.with(|val| matches!(val, Some(Err(_))));
    let errorMessage = Memo::new(move |_| {
        return if let Some(Err(v)) = value.get() {
            Some(v.to_string())
        } else {
            None
        };
    });

    Effect::new(move || {
        if errorMessage().is_some() {
            log!("Auth error {:?}", value.get())
        }
    });
    view! {
        <h2>"Login"</h2>
        <ActionForm action>
            <Show when=move || errorMessage().is_some()>
                <div id="login-error" class="form-error-panel">
                    {errorMessage().unwrap()}
                </div>
            </Show>
            <label>"username"<input name="credentials[username]" /></label>
            <label>"password"<input name="credentials[password]" type="password" /></label>
            <button>Login</button>
            <footer>
                <Suspense fallback=|| ()>
                    {move || Suspend::new(async move {
                        let can_show_reg = registration_available.await.is_ok_and(|r| r);
                        view! {
                            <Show when=move || { can_show_reg }>
                                <p>
                                    <small>
                                        "Don't have an account? "
                                        <A href="../register">"Register"</A>
                                    </small>
                                </p>
                            </Show>
                            <p>
                                <small>Go Home? <A href="/">Home</A></small>
                            </p>
                        }
                    })}
                </Suspense>
            </footer>
        </ActionForm>
    }
}

#[server(endpoint = "auth/login")]
pub async fn try_login(credentials: Credentials) -> Result<(), AuthenticationError> {
    use crate::entities::{User, UserDbError};
    use axum::http::StatusCode;

    let client =
        common::db::use_client().ok_or_else(|| AuthenticationError::InternalServerError)?;

    match User::get_by_credentials(credentials, &client).await {
        Ok(Some(u)) => {
            let auth = crate::session::get_auth_session()
                .await
                .or_else(|s| Err(AuthenticationError::from_server_fn_error(s)))?;

            auth.login_user(u.id);

            // todo: add support for navigating back to an intended url pre login.
            //  would have to have stored it in session during original auth check

            leptos_axum::redirect("/");
            Ok(())
        }
        Ok(None) => {
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::UNAUTHORIZED);
            Err(AuthenticationError::InvalidCredentials)
        }
        Err(e) => match e {
            UserDbError::UsernameExists => {
                // todo: this is a good example of why I should split the errors up, this seems silly to have to deal with
                unreachable!("Username exists error when trying to login")
            }
            UserDbError::UsernameNotExists => {
                let opts = expect_context::<leptos_axum::ResponseOptions>();
                opts.set_status(StatusCode::UNAUTHORIZED);
                Err(AuthenticationError::InvalidCredentials)
            }
            UserDbError::UnknownError => Err(AuthenticationError::InternalServerError),
        },
    }
}

#[server]
pub async fn get_registration_available() -> Result<bool, ServerFnError> {
    use common::state::AppSettings;
    let app_settings = use_context::<AppSettings>().expect("App settings should be available");
    Ok(app_settings.registration_enabled)
}
