use crate::core::auth::User;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone)]
pub struct UserRecord {
    id: Uuid,
    username: String,
    // password_hash: String,
}

impl From<&UserRecord> for User {
    fn from(value: &UserRecord) -> Self {
        Self {
            id: value.clone().id,
            username: value.clone().username
        }
    }
}

impl User {
    // pub fn get(id: Uuid, pool: &PgPool)->Self {
    //     query_as::<_,UserRecord>("SELECT * FROM users WHERE id = ?").bind(id).fetch_one(pool).await?
    // }
    pub async fn get_all(pool: &PgPool) -> Vec<UserRecord> {
        query_as::<_,UserRecord>("SELECT * FROM users").fetch_all(pool).await.expect("Unable to get users")
    }

    // fn get_by_username(username: &str) -> User {}
    // fn hash_password(password: &str) -> Result<(), ()> {
    //     todo!()
    // }
}
