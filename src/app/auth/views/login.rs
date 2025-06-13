use crate::app::auth::{Credentials};
use leptos::prelude::*;
use leptos_router::components::A;

#[server]
pub async fn try_login(credentials: Credentials) -> Result<String, ServerFnError> {
    use crate::app::auth::entity::user::UserRow;
    use axum::http::StatusCode;    
    use leptos::prelude::expect_context;
    use crate::contexts::use_client;
    use crate::app::auth::entity::user::UserDbError;
    use axum_session_auth::AuthSession;
    use axum_session_sqlx::SessionPgPool;
    use uuid::Uuid;
    use sqlx::PgPool;
    use crate::app::auth::{User};

    let client = use_client()
        .ok_or_else(|| ServerFnError::new("Server error"))?;

    match UserRow::get_by_credentials(credentials, &client).await {
        Ok(Some(u)) => {
            let auth = leptos_axum::extract::<AuthSession<User, Uuid, SessionPgPool, PgPool>>().await?;
            
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
        Err(e) => {
            match e {
                UserDbError::UsernameExists => {
                    unreachable!("Username exists error when trying to login")
                }
                UserDbError::UsernameNotExists => {
                    let opts = expect_context::<leptos_axum::ResponseOptions>();
                    opts.set_status(StatusCode::UNAUTHORIZED);
                    Err(ServerFnError::new("Invalid credentials"))
                }
                UserDbError::UnknownError => {
                    Err(ServerFnError::new("System error"))
                }
            }
        }
    }
}

#[component]
pub fn Login() -> impl IntoView {
    let action = ServerAction::<TryLogin>::new();
    let value = Signal::derive(move || {
        action
            .value()
            .get()
            .unwrap_or_else(|| Ok("".into()))
    });

    view! {
        <h2>"Login"</h2>
        <ErrorBoundary fallback=move |errors| {
            view! {
                <p>Error</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect::<Vec<_>>()
                    }}
                </ul>
            }
        }>
            // hack: this is just so the error boundary will actually trigger.
            // I never want to display this. Replace this pattern with an error memo on the value maybe?
            // this pattern is meant to support no JS/progressive flows; but it doesn't seem to work anyway
            <span style="display: none">{value}</span>
        </ErrorBoundary>
        <ActionForm action>
            <label>"username"<input name="credentials[username]" /></label>
            <label>"password"<input name="credentials[password]" type="password" /></label>
            <button>Login</button>
            // todo: wrap this with an app config check to determine if registration is available.
            <small>"Don't have an account? "<A href="../register">"Register"</A></small>
            <small>Go Home? <A href="/">Home</A></small>
        </ActionForm>
    }
}
