use leptos::prelude::*;
use crate::app::auth::{Credentials};

#[server]
pub async fn try_register(credentials: Credentials) -> Result<String, ServerFnError> {
    use crate::app::auth::user::UserRow;
    use axum::http::StatusCode;
    use leptos::prelude::expect_context;
    use crate::db::get_pool;

    // todo: move into main and pass via state/context
    let pool = get_pool().await.ok().unwrap();
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    if let Some(user_row) = UserRow::create(credentials, &pool).await {
        opts.set_status(StatusCode::CREATED);
        // todo: initialise session
        leptos_axum::redirect("/login");
        // return Ok(User::from(user_row))
        return Ok(user_row.id.to_string());
    }
    opts.set_status(StatusCode::BAD_REQUEST);
    Err(ServerFnError::ServerError("Unable to register".into()))
}

#[server]
async fn do_something(
    should_error: Option<String>,
) -> Result<String, ServerFnError> {
    if should_error.is_none() {
        Ok(String::from("Successful submit"))
    } else {
        Err(ServerFnError::ServerError(String::from(
            "You got an error!",
        )))
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

    // Effect::new_isomorphic(move |_| {
    //     log!("Got value = {:?}", value.get());
    // });
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