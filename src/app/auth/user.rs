use super::{Credentials, User};
use crate::app::auth::utils::hash_password;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum_session_auth::Authentication;
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

// todo: split into errors for kind of thing we're doing to try to avoid unreachable error states elsewhere
#[derive(Error, Debug)]
pub enum UserDbError {
    #[error("Username is taken")]
    UsernameExists,
    #[error("Username does not exist")]
    UsernameNotExists,
    #[error("Unknown error")]
    UnknownError,
    // CreateUserValidationErrors
        // - Username Exists
        // - Validation failed Username length
        // - Validation failed Password length
        // - Validation failed Password strength
}

impl UserRow {
    async fn get_by_id(user_id: Uuid, pool: &PgPool)-> Result<Option<UserRow>, UserDbError> {
        match query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await {
            Err(e) => {
                println!("get_by_id: {e}");
                Err(UserDbError::UnknownError)
            },
            Ok(u) => {
                Ok(Some(u))
            }
        }
    }
    async fn username_exists(username: &str, pool: &PgPool) -> Result<bool, UserDbError> {
        let username_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"
        ).bind(username.to_string()).fetch_one(pool).await.map_err(|e1| {
            println!("Error: {e1:?}");
            UserDbError::UnknownError
        })?;
        Ok(username_exists)
    }
    pub async fn create(credentials: Credentials, pool: &PgPool) -> Result<UserRow, UserDbError> {
        // todo: validations
        //  - no empty values
        //  - minimum pw length/strength validations 
        
        let username_taken: bool =  Self::username_exists(&credentials.username, pool).await?;

        if username_taken {
            return Err(UserDbError::UsernameExists)
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
    pub async fn get_by_username(username: String, pool: &PgPool) -> Result<Option<UserRow>, UserDbError> {
        let username_exists: bool =  Self::username_exists(&username, pool).await?;

        if !username_exists {
            return Err(UserDbError::UsernameNotExists)
        };
        
        match query_as::<_, UserRow>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await {
            Err(e) => {
                println!("get_by_username: {e}");
                Err(UserDbError::UnknownError)
            },
            Ok(u) => {
                Ok(Some(u))
            }
        }
    }
    
    pub async fn get_by_credentials(credentials: Credentials, pool: &PgPool) -> Result<Option<UserRow>, UserDbError>  {
        match Self::get_by_username(credentials.username, pool).await {
            Ok(Some(u)) => {
                let expected_hash = PasswordHash::new(&u.password_hash).expect("Unable to hash user password hash");
                if Argon2::default().verify_password(credentials.password.as_bytes(), &expected_hash).is_ok() {
                    return Ok(Some(u))
                }
                Ok(None)
            }
            Ok(None) => {
                // note: doing a check even with no user row returned to minimise timing differences
                // between not found and found user checks and avoid potential information leak
                // about existence of user existence
                // rethink if this is actually meaningful if we're using usernames and not emails to login; as we have to report  existence errors in registration anyway?
                // Why is it okay to expose this info durng registration but not during login?
                let password_hash = hash_password(&credentials.password).await.unwrap();
                PasswordHash::new(&password_hash).expect("Unable to hash dummy password hash");
                Ok(None)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            id: value.id,
            username: value.username,
            anonymous: false
        }
    }
}

#[async_trait::async_trait]
impl Authentication<User, Uuid, PgPool> for User {
    async fn load_user(userid: Uuid, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        match UserRow::get_by_id(userid, pool.unwrap()).await {
            Ok(Some(u)) => {
                Ok(User::from(u))
            },
            Ok(None) => {
                Ok(User {
                    id: Uuid::nil(),
                    username: String::from(""),
                    anonymous: true
                })
            }
            Err(_) => {
                Err(anyhow::anyhow!("Cannot get user"))
            }            
        }
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }
    
    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}