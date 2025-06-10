use leptos::prelude::*;
use leptos::task::spawn_local;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::core::auth::ssr::{get_user};

#[derive(Clone, Serialize,Deserialize)]
pub struct User {
    id: Uuid,
    username: String,
}

pub mod ssr {
    use leptos::prelude::ServerFnError;
    use leptos::server;
    use crate::core::auth::User;

    #[server]
    pub async fn get_user() -> Result<User, ServerFnError> {
        Ok(User {
            id: uuid::Uuid::new_v4(),
            username: "Bobert".into(),
        })
    }
}


#[component]
pub fn UserView() -> impl IntoView {
    let (user, set_user) = signal(User {
        id: Uuid::default(),
        username: "".into()
    });

    view! {
        <p>User</p>
        <button on:click=move |_| {
            spawn_local(async move {
                let u = get_user().await.expect("oh no");
                set_user.set(u);
            })
        }>GetUser</button>
        <p>{move || user.get().id.to_string()}</p>
        <p>{move || user.get().username}</p>
    }
}
