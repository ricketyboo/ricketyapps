use leptos::prelude::*;

#[component]
pub fn Logout() -> impl IntoView {
    OnceResource::new_blocking(logout());
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::auth;
    use axum_session_sqlx::SessionPgPool;
    let auth = leptos_axum::extract::<
        axum_session_auth::AuthSession<
            auth::AuthSessionUser,
            uuid::Uuid,
            SessionPgPool,
            sqlx::PgPool,
        >,
    >()
    .await?;
    auth.logout_user();
    leptos_axum::redirect("/login");
    Ok(())
}
