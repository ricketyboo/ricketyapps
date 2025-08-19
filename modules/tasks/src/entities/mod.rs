use crate::dto::{CreateTaskInput, TaskListItem};
use sqlx::types::chrono;
use uuid::Uuid;
use welds::prelude::*;

#[derive(Debug, WeldsModel)]
#[welds(table = "tasks")]
pub struct Task {
    #[welds(primary_key)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    #[welds(readonly)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[welds(readonly)]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[welds(readonly)]
    pub completed: bool,
}

impl Task {
    pub(crate) async fn create(create_task: CreateTaskInput, owner_id: &Uuid) -> DbState<Task> {
        let mut task = Task::new();
        task.title = create_task.title;
        task.content = create_task.content;
        task.owner_id = *owner_id;
        task
    }
}

impl From<Task> for TaskListItem {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
            completed_at: value.completed_at,
        }
    }
}
