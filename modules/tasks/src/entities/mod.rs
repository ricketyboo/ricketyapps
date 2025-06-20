use crate::dto::{CreateTask, TaskListItem};
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
}

impl Task {
    pub(crate) async fn from_dto_for_owner(
        create_task: CreateTask,
        owner_id: &Uuid,
    ) -> DbState<Task> {
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
        }
    }
}
