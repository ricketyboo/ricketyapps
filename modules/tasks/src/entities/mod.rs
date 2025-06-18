use crate::dto::{CreateTask, TaskListItem};
use uuid::Uuid;
use welds::prelude::*;

#[derive(Debug, WeldsModel)]
#[welds(table = "tasks")]
pub struct Task {
    #[welds(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
}

impl From<CreateTask> for DbState<Task> {
    fn from(value: CreateTask) -> Self {
        let mut task = Task::new();
        // todo: owner_id from session
        task.title = value.title;
        task.content = value.content;
        task
    }
}

impl From<Task> for TaskListItem {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
        }
    }
}
