use crate::auth::utils::hash_password;
use crate::auth::{Credentials, User};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum_session_auth::Authentication;
use sqlx::PgPool;
use uuid::Uuid;

use thiserror::Error;
use welds::WeldsModel;
use welds::connections::postgres::PostgresClient;

// todo: rename this and work out how to avoid confusion between the non SSR User that Auth Uses and this User Entity
#[derive(sqlx::FromRow, Debug, WeldsModel)]
#[welds(table = "users")]
pub struct UserRow {
    #[welds(primary_key)]
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
    async fn username_exists(username: &str, client: &PostgresClient) -> Result<bool, UserDbError> {
        let username_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(username.to_string())
                .fetch_one(client.as_sqlx_pool())
                .await
                .map_err(|e1| {
                    println!("Error: {e1:?}");
                    UserDbError::UnknownError
                })?;
        Ok(username_exists)
    }
    pub async fn create(
        credentials: Credentials,
        client: &PostgresClient,
    ) -> Result<UserRow, UserDbError> {
        // todo: validations
        //  - no empty values
        //  - minimum pw length/strength validations

        let username_taken: bool = Self::username_exists(&credentials.username, client).await?;

        if username_taken {
            return Err(UserDbError::UsernameExists);
        };

        let password_hash = hash_password(&credentials.password).await.unwrap();
        let mut model = UserRow::new();
        model.username = credentials.username;
        model.password_hash = password_hash;
        match model.save(client).await {
            Ok(_) => Ok(model.into_inner()),
            Err(e) => {
                println!("{e:?}");
                Err(UserDbError::UnknownError)
            }
        }
    }
    pub async fn get_by_username(
        username: String,
        client: &PostgresClient,
    ) -> Result<Option<UserRow>, UserDbError> {
        let username_exists: bool = Self::username_exists(&username, client).await?;

        if !username_exists {
            return Err(UserDbError::UsernameNotExists);
        };
        match UserRow::where_col(move |u| u.username.equal(username.clone()))
            .fetch_one(client)
            .await
        {
            Ok(row) => Ok(Some(row.into_inner())),
            Err(e) => {
                println!("get_by_username: {e}");
                Err(UserDbError::UnknownError)
            }
        }
    }

    pub async fn get_by_credentials(
        credentials: Credentials,
        client: &PostgresClient,
    ) -> Result<Option<User>, UserDbError> {
        match Self::get_by_username(credentials.username, client).await {
            Ok(Some(u)) => {
                let expected_hash =
                    PasswordHash::new(&u.password_hash).expect("Unable to hash user password hash");
                if Argon2::default()
                    .verify_password(credentials.password.as_bytes(), &expected_hash)
                    .is_ok()
                {
                    return Ok(Some(User::from(u)));
                }
                Ok(None)
            }
            Ok(None) => {
                // note: doing a check even with no user row returned to minimise timing differences
                // between not found and found user checks and avoid potential information leak
                // about existence of user existence
                // rethink if this is actually meaningful if we're using usernames and not emails to login; as we have to report  existence errors in registration anyway?
                // Why is it okay to expose this info during registration but not during login?
                let password_hash = hash_password(&credentials.password).await.unwrap();
                PasswordHash::new(&password_hash).expect("Unable to hash dummy password hash");
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            id: value.id,
            username: value.username,
            anonymous: false,
        }
    }
}

#[async_trait::async_trait]
impl Authentication<User, Uuid, PgPool> for User {
    async fn load_user(userid: Uuid, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        // because auth_session_axum expects to be using raw sqlx pools we stil have to pass that in.
        // but because welds wants a client we have to convert it from the pool.
        let welds_client: PostgresClient = pool.unwrap().clone().into();
        match UserRow::find_by_id(&welds_client, userid).await {
            Ok(Some(u)) => Ok(u.into_inner().into()),
            Ok(None) => Ok(User {
                id: Uuid::nil(),
                username: String::from(""),
                anonymous: true,
            }),
            Err(_) => Err(anyhow::anyhow!("Cannot get user")),
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
