use leptos::prelude::*;

#[component]
pub fn LogoutPage() -> impl IntoView {
    OnceResource::new_blocking(logout());
}

#[server(endpoint = "auth/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::dto::AuthSessionUser;
    use axum_session_sqlx::SessionPgPool;
    let auth = leptos_axum::extract::<
        axum_session_auth::AuthSession<AuthSessionUser, uuid::Uuid, SessionPgPool, sqlx::PgPool>,
    >()
    .await?;
    auth.logout_user();
    leptos_axum::redirect("/login");
    Ok(())
}
