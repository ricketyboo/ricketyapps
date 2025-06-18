use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthSessionUser {
    pub id: Uuid,
    pub username: String,
    pub anonymous: bool,
}
