use crate::dto::TaskListItem;
use leptos::prelude::*;
use leptos::{IntoView, component, server, view};
use uuid::Uuid;

#[component]
pub(super) fn TaskListItem(task: TaskListItem) -> impl IntoView {
    let complete = RwSignal::new(task.completed_at.is_some());
    let action = ServerAction::<SetTaskStatus>::new();
    Effect::new(move |v| {
        let completed = complete.read();
        // don't run this on the first load, or we'll re-update everything as the list loads
        if v != None {
            action.dispatch(SetTaskStatus {
                task_id: task.id,
                completed: completed.clone(),
            });
        }
    });
    view! {
        <label>
            <input type="checkbox" bind:checked=complete/>
            {task.title}
        </label>
    }
}
#[server]
pub(self) async fn set_task_status(task_id: Uuid, completed: bool) -> Result<(), ServerFnError> {
    use crate::entities::Task;
    let client = common::db::use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
    if let Some(mut task) = Task::find_by_id(&client, task_id).await? {
        if completed {
            task.completed_at = Some(chrono::Utc::now());
        } else {
            task.completed_at = None;
        }
        task.save(&client).await?;
    }
    Ok(())
}
