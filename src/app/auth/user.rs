use super::{Credentials, User};
use sqlx::{PgPool, query_as};
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct UserRow {
    id: Uuid,
    username: String,
    // todo: SecretString
    password_hash: String,
}

impl UserRow {
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
        // todo: run the hash on the input password
        let password_hash = credentials.password;
        match Self::get_by_username(credentials.username, pool).await {
            None => {None}
            Some(u) => {
                if u.password_hash == password_hash {
                    return Some(User::from(u))
                }
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
