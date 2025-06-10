use super::{Credentials, User};
use crate::app::auth::utils::hash_password;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use thiserror::Error;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct UserRow {
    pub(crate) id: Uuid,
    username: String,
    // todo: SecretString
    password_hash: String,
}

#[derive(Error, Debug)]
pub enum UserDbError {
    #[error("Username is taken")]
    UserExists,
    #[error("System error")]
    UnknownError,
}


impl UserRow {
    pub async fn create(credentials: Credentials, pool: &PgPool) -> Result<UserRow, UserDbError> {
        // todo: validations
        
        let username_taken: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"
        ).bind(credentials.username.clone()).fetch_one(pool).await.map_err(|e1| {
            println!("Error: {e1:?}");
            UserDbError::UnknownError
        })?;

        if username_taken {
            return Err(UserDbError::UserExists)
        };

        let password_hash = hash_password(&credentials.password).await.unwrap();

        match query_as::<_, UserRow>("INSERT INTO users (username, password_hash) VALUES ( $1,  $2) returning *")
            .bind(credentials.username)
            .bind(password_hash)
            .fetch_one(pool)
            .await {
            Err(e) => {
                println!("{e}");
                Err(UserDbError::UnknownError)
            },
            Ok(u) => {
                Ok(u)
            }
        }
    }
    pub async fn get_by_username(username: String, pool: &PgPool) -> Option<UserRow> {
        match query_as::<_, UserRow>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await {
            Err(e) => {
                println!("{e}");
                None
            },
            Ok(u) => {
                Some(u)
            }
        }
    }

    pub async fn get_by_credentials(credentials: Credentials, pool: &PgPool) -> Option<User> {
        match Self::get_by_username(credentials.username, pool).await {
            Some(u) => {
                let expected_hash = PasswordHash::new(&u.password_hash).expect("Unable to hash user password hash");
                if Argon2::default().verify_password(credentials.password.as_bytes(), &expected_hash).is_ok() {
                    return Some(User::from(u))
                }
                None
            }
            None => {
                // note: doing a check even with no user row returned to minimise timing differences
                // between not found and found user checks and avoid potential information leak
                // about existence of user existence
                let password_hash = hash_password(&credentials.password).await.unwrap();
                PasswordHash::new(&password_hash).expect("Unable to hash dummy password hash");
                None
            }
        }
    }
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}
