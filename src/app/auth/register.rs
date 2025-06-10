use leptos::logging::log;
use leptos::prelude::*;
use crate::app::auth::{Credentials, User};

#[server]
pub async fn try_register(credentials: Credentials) -> Result<User, ServerFnError> {
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
        return Ok(User::from(user_row))
    }
    opts.set_status(StatusCode::BAD_REQUEST);
    Err(ServerFnError::ServerError("Unable to register".into()))
}

#[component]
pub fn Register() -> impl IntoView {
    let submit_action = ServerAction::<TryRegister>::new();
    // let value = Signal::derive(move || {
    //     submit_action
    //         .value()
    //         .get()
    //         .unwrap_or_else(|| Ok(User::default()))
    // });
    // Effect::new_isomorphic(move |_| {
    //     log!("Got value = {:?}", value.get());
    // });
    view! {
        <h2>"Register"</h2>
        // <ErrorBoundary fallback=move |error| {
        // view! {
        // <p>Error</p>
        // <ul>
        // {move || {
        // error
        // .get()
        // .into_iter()
        // .map(|(_, e)| view! { <li>{e.to_string()}</li> })
        // .collect::<Vec<_>>()
        // }}
        // 
        // </ul>
        // }
        // }>
        // <pre>{move || value.get().unwrap().id.to_string()}</pre>
        <ActionForm action=submit_action>
            <label>"username"<input name="credentials[username]" /></label>
            <label>"password"<input name="credentials[password]" type="password" /></label>
            <button type="submit">Register</button>
        </ActionForm>
        // </ErrorBoundary>
    }
}