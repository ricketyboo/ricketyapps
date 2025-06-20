use crate::dto::{CreateTaskInput, TaskListItem};
use sqlx::types::chrono;
use utility_types::Omit;
use uuid::Uuid;
use welds::prelude::*;

#[derive(Debug, WeldsModel, Omit)]
// slightly silly workaround for https://github.com/weldsorm/welds/issues/122 to avoid writing epoch into the created_at DB column
#[omit(
    arg(
        ident = CreateTask,
        fields(created_at),
        derive(Debug, WeldsModel),
        forward_attrs(welds)
    )
)]
#[welds(table = "tasks")]
pub struct Task {
    #[welds(primary_key)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Task {
    pub(crate) async fn from_dto_for_owner(
        create_task: CreateTaskInput,
        owner_id: &Uuid,
    ) -> DbState<CreateTask> {
        let mut task = CreateTask::new();
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
