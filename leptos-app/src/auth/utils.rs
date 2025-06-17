// todo: proper error returns
// todo: move this into the user entity; there's no use for this to be public
pub async fn hash_password(password: &str) -> Result<String, &'static str> {
    use argon2::Algorithm::Argon2id;
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, Params, PasswordHasher, Version};

    use rand::thread_rng;

    let salt = SaltString::generate(&mut thread_rng());
    let password_hash = Argon2::new(
        Argon2id,
        Version::V0x13,
        // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
        Params::new(19456, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string();

    Ok(password_hash)
}

pub mod session {
    use crate::auth::User;
    use axum_session_auth::AuthSession;
    use axum_session_auth::Authentication;
    use axum_session_sqlx::SessionPgPool;
    use leptos::prelude::ServerFnErrorErr;
    use sqlx::PgPool;
    use uuid::Uuid;

    pub async fn get_auth_session()
    -> Result<AuthSession<User, Uuid, SessionPgPool, PgPool>, ServerFnErrorErr> {
        leptos_axum::extract::<AuthSession<User, Uuid, SessionPgPool, PgPool>>().await
    }

    pub async fn get_current_user() -> Result<Option<User>, ServerFnErrorErr> {
        let auth_session = get_auth_session().await?;
        Ok(auth_session.current_user)
    }

    pub async fn is_user_logged_in() -> Result<bool, ServerFnErrorErr> {
        Ok(get_current_user()
            .await
            .is_ok_and(|u| u.is_some_and(|u| u.is_authenticated())))
    }
}
