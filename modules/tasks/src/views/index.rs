use super::list::TaskListItem;
use crate::dto::{CreateTaskInput, TaskListItem};

use leptos::html::Form;
use leptos::prelude::*;

#[component]
pub fn TaskIndex() -> impl IntoView {
    let action = ServerAction::<AddTask>::new();
    // todo: optimistic updates
    let tasks_resource = Resource::new(move || action.version().get(), move |_| get_tasks());
    let form_ref: NodeRef<Form> = NodeRef::new();
    Effect::new(move || {
        action.version().get();
        form_ref.get().unwrap().reset();
    });
    view! {
        <h1>Tasks</h1>
        <ActionForm action node_ref=form_ref>
            <label>"Task"<input name="create_task[title]" /></label>
            <label>"Description"<input name="create_task[content]" /></label>
            <button>"Submit"</button>
        </ActionForm>
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
                                        <TaskListItem task/>
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

#[server]
pub async fn add_task(create_task: CreateTaskInput) -> Result<(), ServerFnError> {
    use crate::entities::Task;
    let client = common::db::use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
    let owner = auth::session::get_current_user()
        .await?
        .expect("Unable to get current user");
    let mut task = Task::from_dto_for_owner(create_task, &owner.id).await;
    task.save(&client).await?;
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
    let tasks_db_state = Task::all()
        .where_col(|t| t.owner_id.equal(owner.id))
        .order_by_asc(|t| t.created_at)
        .run(&client)
        .await?;
    let tasks: Vec<TaskListItem> = tasks_db_state
        .into_inners()
        .into_iter()
        .map(TaskListItem::from)
        .collect();
    Ok(tasks)
}
