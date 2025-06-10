use argon2::{Argon2, PasswordHash, PasswordVerifier};
use super::{Credentials, User};
use sqlx::{PgPool, query_as};
use uuid::Uuid;
use crate::app::auth::utils::hash_password;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct UserRow {
    id: Uuid,
    username: String,
    // todo: SecretString
    password_hash: String,
}

impl UserRow {
    pub async fn create(credentials: Credentials, pool: &PgPool) -> Option<UserRow> {
        // todo: error handling
        let password_hash = hash_password(&credentials.password).await.unwrap();

        match query_as::<_, UserRow>("INSERT INTO users (username, password_hash) VALUES ( $1,  $2) returning *")
            .bind(credentials.username)
            .bind(password_hash)
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
                let expected_hash = PasswordHash::new(&u.password_hash).expect("Unable to hash user passhash");
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
                PasswordHash::new(&password_hash).expect("Unable to hash dummy passhash");
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
