use crate::dto::CreateTask;
use leptos::prelude::*;

#[component]
pub fn TaskIndex() -> impl IntoView {
    let action = ServerAction::<AddTask>::new();
    view! {
        <h1>Tasks</h1>
        <ActionForm action>
            <label>"Task"<input name="create_task[title]"/></label>
            <label>"Description"<input name="create_task[content]"/></label>
            <button>"Submit"</button>
        </ActionForm>
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
    // Return Result
    Ok(())
}
