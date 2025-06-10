use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub(crate) mod ssr;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
