use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub mod ssr;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
