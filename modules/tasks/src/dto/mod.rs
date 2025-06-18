use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub content: Option<String>,
}
