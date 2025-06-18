use crate::dto::{CreateTask, TaskListItem};
use leptos::prelude::*;

#[component]
pub fn TaskIndex() -> impl IntoView {
    let action = ServerAction::<AddTask>::new();
    // todo: optimistic updates and refetching on success
    let tasks_resource = OnceResource::new(get_tasks());
    view! {
        <h1>Tasks</h1>
        <ActionForm action>
            <label>"Task"<input name="create_task[title]" /></label>
            <label>"Description"<input name="create_task[content]" /></label>
            <button>"Submit"</button>
        </ActionForm>
        <h3>Task List</h3>

        <Suspense fallback=|| {
            view! { <p>Loading tasks</p> }
        }>
            {move || Suspend::new(async move {
                if let Ok(tasks) = tasks_resource.await {
                    view! {
                        <ul id="task-list">
                            <For each=move || tasks.clone() key=|t| t.id let(task)>
                                <li>
                                    <label>
                                        <input type="checkbox" disabled />
                                        {task.title}
                                    </label>

                                </li>
                            </For>
                        </ul>
                    }
                        .into_any()
                } else {
                    view! { <p>Unable to load tasks</p> }.into_any()
                }
            })}
        </Suspense>
    }
}

#[server]
pub async fn add_task(create_task: CreateTask) -> Result<(), ServerFnError> {
    use crate::entities::Task;
    use welds::state::DbState;
    // Use task entity to create
    let mut task: DbState<Task> = create_task.into();
    let client = common::db::use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
    task.save(&client).await?;
    Ok(())
}

#[server]
pub async fn get_tasks() -> Result<Vec<TaskListItem>, ServerFnError> {
    use crate::entities::Task;
    use welds::prelude::*;
    let client = common::db::use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
    let tasks_db_state = Task::all().run(&client).await?;
    let tasks: Vec<TaskListItem> = tasks_db_state
        .into_inners()
        .into_iter()
        .map(TaskListItem::from)
        .collect();
    Ok(tasks)
}
