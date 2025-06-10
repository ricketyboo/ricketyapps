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
    let users_resource = Resource::new_blocking(move || "", |_| get_users());

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let users = users_resource.await;
                view! {
                    <For each=move || users.clone().unwrap_or_default() key=|u| u.id let(user)>
                        <div>
                            <p>{move || user.id.to_string()}</p>
                            <p>{move || user.username.to_string()}</p>
                        </div>
                    </For>
                }
            })}
        </Suspense>
    }
}
