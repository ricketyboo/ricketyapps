use super::list::{TaskList, get_tasks};
use crate::dto::CreateTaskInput;

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
        <TaskList tasks_resource/>
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
