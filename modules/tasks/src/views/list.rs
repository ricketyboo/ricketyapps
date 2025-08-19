use crate::dto::TaskListItem;
use leptos::prelude::*;
use leptos::{IntoView, component, server, view};
use uuid::Uuid;

#[component]
pub(super) fn TaskList(
    tasks_resource: Resource<Result<Vec<TaskListItem>, ServerFnError>>,
) -> impl IntoView {
    view! {
        <h3>Task List</h3>

        <Suspense fallback=|| {
            view! { <p>Loading tasks</p> }
        }>
            {move || Suspend::new(async move {
                match tasks_resource.await {
                    Ok(tasks) if !tasks.is_empty() => {
                        view! {
                            <ul id="task-list">
                                <For each=move || tasks.clone() key=|t| t.id let(task)>
                                    <li>
                                        <TaskListItem task />
                                    </li>
                                </For>
                            </ul>
                        }
                            .into_any()
                    }
                    Ok(_) => view! { <p>No tasks!</p> }.into_any(),
                    Err(_) => view! { <p>Error loading your tasks</p> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn TaskListItem(task: TaskListItem) -> impl IntoView {
    let complete = RwSignal::new(task.completed);
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
            <input type="checkbox" bind:checked=complete />
            {task.title}
        </label>
    }
}
#[server]
async fn set_task_status(task_id: Uuid, completed: bool) -> Result<(), ServerFnError> {
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

#[server]
pub async fn get_tasks() -> Result<Vec<TaskListItem>, ServerFnError> {
    use crate::entities::Task;
    use welds::prelude::*;
    let client = common::db::use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
    let owner = auth::session::get_current_user()
        .await?
        .expect("Unable to get current user");
    let tasks: Vec<Task> = Task::all()
        .where_col(|t| t.owner_id.equal(owner.id))
        .select_all()
        .order_by_asc(|t| t.completed)
        .order_by_desc(|t| t.completed_at)
        .order_by_asc(|t| t.created_at)
        .run(&client)
        .await?
        .collect_into()?;
    let tasks: Vec<TaskListItem> = tasks
        // .into_inners()
        .into_iter()
        .map(TaskListItem::from)
        .collect();
    Ok(tasks)
}
