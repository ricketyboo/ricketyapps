use leptos::prelude::*;

use crate::core::auth::ssr::get_users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct User {
    id: Uuid,
    username: String,
}

pub mod ssr {
    use crate::core::auth::User;
    use leptos::prelude::ServerFnError;
    use leptos::server;

    #[server]
    pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        Ok(vec![
            User {
                id: uuid::Uuid::new_v4(),
                username: "Bobert".into(),
            },
            User {
                id: uuid::Uuid::new_v4(),
                username: "Susandy".into(),
            },
        ])
    }
}

#[component]
pub fn UserView() -> impl IntoView {
    let users_data = OnceResource::new(get_users());

    view! {
        <Suspense fallback=|| {
            view! { <p>Loading...</p> }
        }>
            {move || Suspend::new(async move {
                if let Ok(users) = users_data.await {
                    view! {
                        <For each=move || users.clone() key=|u| u.id let(user)>
                            <p>{user.username}</p>
                        </For>
                    }
                        .into_any()
                } else {
                    view! { <p>"no users"</p> }
                        .into_any()
                }
            })}
        </Suspense>
    }
}
