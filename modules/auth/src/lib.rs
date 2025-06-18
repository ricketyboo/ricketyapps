pub mod dto;
#[cfg(feature = "ssr")]
pub mod entities;
#[cfg(feature = "ssr")]
pub mod mappers;
pub mod views;

#[cfg(feature = "ssr")]
pub mod session {
    use crate::dto::AuthSessionUser;
    use axum_session_auth::AuthSession;
    use axum_session_auth::Authentication;
    use axum_session_sqlx::SessionPgPool;
    use leptos::prelude::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    pub async fn get_auth_session()
    -> Result<AuthSession<AuthSessionUser, Uuid, SessionPgPool, PgPool>, ServerFnErrorErr> {
        leptos_axum::extract::<AuthSession<AuthSessionUser, Uuid, SessionPgPool, PgPool>>().await
    }

    pub async fn get_current_user() -> Result<Option<AuthSessionUser>, ServerFnErrorErr> {
        let auth_session = get_auth_session().await?;
        Ok(auth_session.current_user)
    }

    pub async fn is_user_logged_in() -> Result<bool, ServerFnErrorErr> {
        Ok(get_current_user()
            .await
            .is_ok_and(|u| u.is_some_and(|u| u.is_authenticated())))
    }
}
