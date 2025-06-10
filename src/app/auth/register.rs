use leptos::prelude::*;

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
                UserDbError::UserExists => {
                    opts.set_status(StatusCode::CONFLICT);
                    Err(ServerFnError::ServerError(e.to_string()))
                },
                UserDbError::UnknownError => {
                    opts.set_status(StatusCode::BAD_REQUEST);
                    Err(ServerFnError::ServerError("Unable to register".into()))
                }
            }
        }
    }
}

#[component]
pub fn Register() -> impl IntoView {
    let submit_action = ServerAction::<TryRegister>::new();
    let value = Signal::derive(move || {
        submit_action
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
            <span style="display: none">{value}</span>
        </ErrorBoundary>
        <ActionForm action=submit_action>
            <label>"username"<input name="credentials[username]" /></label>
            <label>"password"<input name="credentials[password]" type="password" /></label>
            <button type="submit">Register</button>
        </ActionForm>
    }
}