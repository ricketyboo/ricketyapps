
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod login;
mod register;
#[cfg(feature = "ssr")]
mod user;
#[cfg(feature = "ssr")]
mod utils;



pub mod views {
    pub use super::login::Login;
    pub use super::register::Register;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub anonymous: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String,
}
