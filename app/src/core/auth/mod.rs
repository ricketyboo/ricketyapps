use leptos::prelude::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
mod ssr;

#[server]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use crate::ssr::AppState;
    match with_context::<AppState, _>(|state| state.pool.clone())
        .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into())) {
        Ok(pool) => {
            let users = User::get_all(&pool).await;
            let users_mapped = users.iter().map(User::from).collect();
            Ok(users_mapped)
        }, Err(e) => Err(e)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct User {
    id: Uuid,
    username: String,
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
                            <UserListItem user=user />
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

#[component]
fn UserListItem(user: User) -> impl IntoView {
    use leptos_router::components::A;
    view! {
        <p>
            <A href=user.id.to_string()>{user.username}</A>
        </p>
    }
}
