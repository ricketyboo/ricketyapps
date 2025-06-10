use leptos::prelude::*;
use leptos_router::components::A;
use super::{Credentials};

#[server]
pub async fn try_register(credentials: Credentials) -> Result<String, ServerFnError> {
    use crate::app::auth::user::UserRow;
    use axum::http::StatusCode;
    use leptos::prelude::expect_context;
    use crate::contexts::use_pool;

    use super::user::UserDbError;

    let pool = use_pool()
        .ok_or_else(|| ServerFnError::new("Server error"))?;

    let opts = expect_context::<leptos_axum::ResponseOptions>();

    match UserRow::create(credentials, &pool).await {
        Ok(user_row) => {
            opts.set_status(StatusCode::CREATED);
            // todo: initialise session
            leptos_axum::redirect("/login");
            // return Ok(User::from(user_row))
            Ok(user_row.id.to_string())
        }
        Err(e) => {
            match e {
                UserDbError::UsernameExists => {
                    opts.set_status(StatusCode::CONFLICT);
                    Err(ServerFnError::new(e.to_string()))
                }
                UserDbError::UnknownError => {
                    opts.set_status(StatusCode::BAD_REQUEST);
                    Err(ServerFnError::new("Unable to register"))
                },
                // todo: this probably  means I should split out errors based on what journey I'm oninstead of pushing them all into one enum
                UserDbError::UsernameNotExists => unreachable!("User not exists error gotten on register flow")
            }
        }
    }
}

#[component]
pub fn Register() -> impl IntoView {
    let action = ServerAction::<TryRegister>::new();
    let value = Signal::derive(move || {
        action
            .value()
            .get()
            .unwrap_or_else(|| Ok(String::new()))
    });
    
    view! {
        <h2>"Register"</h2>
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
            <button type="submit">Register</button>
            <small>Already have an account? <A href="../login">Login</A></small>
            <small>Go Home? <A href="/">Home</A></small>
        </ActionForm>
    }
}
