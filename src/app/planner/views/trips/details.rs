#[cfg(feature = "ssr")]
use crate::app::planner::entity::Trip;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[component]
pub fn TripDetailView() -> impl IntoView {
    let params = use_params_map();

    let details = Resource::new(
        move || params.read().get("id"),
        |id| get_trip(Uuid::parse_str(&id.unwrap_or(Uuid::nil().to_string())).expect("Not a Uuid")),
    );

    view! {
        <Suspense fallback=move||view! {<p>Loading...</p>}>
            {move || Suspend::new(async move {
                let trip = details.await.unwrap();
                view! {
                    <p>Details</p>
                    <p>Name: {trip.name}</p>
                }
            })}
        </Suspense>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TripDetails {
    name: String
}

#[cfg(feature = "ssr")]
impl From<Trip> for TripDetails {
    fn from(value: Trip) -> Self {
        Self {
            name: value.name
        }
    }
}

#[server]
async fn get_trip(id: Uuid) -> Result<TripDetails, ServerFnError> {
    println!("get_trip: {id:?}");
    use crate::contexts::use_client;
    let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;

    let trip = Trip::find_by_id(&client, id).await?.expect("No row");
    Ok(trip.into_inner().into())
}
