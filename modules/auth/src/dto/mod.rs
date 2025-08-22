use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// todo: ideally read these validation rules from the entity
#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(length(min = 5))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthSessionUser {
    pub id: Uuid,
    pub username: String,
    pub anonymous: bool,
}
