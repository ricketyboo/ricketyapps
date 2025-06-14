use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[component]
pub(super) fn TripList() -> impl IntoView {
    let items = OnceResource::new(get_trips());
    view! {
        <Suspense fallback=move||view! {<p>Loading</p>}>
            {move || Suspend::new(async move {
                if let Ok(trips) = items.await {
                    view! {
                        <ul>
                            <For each=move || trips.clone() key=|trip| trip.id let(trip)>
                                <li><A href=trip.id.to_string()>{trip.name}</A></li>
                            </For>
                        </ul>
                    }.into_any()
                } else {
                    view! {
                        <ul/>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TripListItem {
    id: Uuid,
    name: String,
}

#[cfg(feature = "ssr")]
use crate::app::planner::entity::Trip;
#[cfg(feature = "ssr")]
impl From<Trip> for TripListItem {
    fn from(value: Trip) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[server]
async fn get_trips() -> Result<Vec<TripListItem>, ServerFnError> {
    use welds::prelude::*;

    use crate::app::auth::User;

    use crate::contexts::use_client;
    let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;

    use axum_session_auth::AuthSession;
    use axum_session_sqlx::SessionPgPool;
    use sqlx::PgPool;
    use uuid::Uuid;
    let auth = leptos_axum::extract::<AuthSession<User, Uuid, SessionPgPool, PgPool>>().await?;

    let user = auth.current_user.expect("No active user");

    let trips = Trip::where_col(|t| t.owner_id.equal(user.id))
        .run(&client)
        .await?;
    let trip_list = trips
        .into_inners()
        .into_iter()
        .map(TripListItem::from)
        .collect();
    Ok(trip_list)
}
