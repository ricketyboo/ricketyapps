use crate::dto::TaskListItem;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::{IntoView, component, server, view};
use uuid::Uuid;

#[component]
pub(super) fn TaskListItem(task: TaskListItem) -> impl IntoView {
    let complete = RwSignal::new(task.completed_at.is_some());
    let action = ServerAction::<SetTaskStatus>::new();
    Effect::new(move |v| {
        log!("v {:?}", v);
        if v != None {
            action.dispatch(SetTaskStatus {
                task_id: task.id,
                completed: complete.get(),
            });
        } else {
            log!("First run, task is {}", complete.read())
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
