use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskListItem {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
}
