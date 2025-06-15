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
    use crate::app::auth::utils::get_current_user;
    let current_user = get_current_user()
        .await
        .map_err(|_| ServerFnError::new("Unable to check current user auth"))?;

    match current_user {
        Some(user) => {
            use crate::contexts::use_client;
            use welds::prelude::*;
            let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
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
        None => {
            use axum::http::StatusCode;
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("No current user"))
        }
    }
}
