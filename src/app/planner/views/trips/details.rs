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
        <Suspense fallback=move || view! { <p>Loading...</p> }>
            <ErrorBoundary fallback=|_| {
                view! { "Error" }
            }>
                {move || Suspend::new(async move {
                    match details.await {
                        Ok(trip) => {
                            view! {
                                <p>Details</p>
                                <p>Name: {trip.name}</p>
                            }
                                .into_any()
                        }
                        Err(e) => {
                            view! {
                                <p>Details</p>
                                <p>{e.to_string()}</p>
                            }
                                .into_any()
                        }
                    }
                })}
            </ErrorBoundary>
        </Suspense>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TripDetails {
    name: String,
}

#[cfg(feature = "ssr")]
impl From<Trip> for TripDetails {
    fn from(value: Trip) -> Self {
        Self { name: value.name }
    }
}

#[server]
async fn get_trip(id: Uuid) -> Result<TripDetails, ServerFnError> {
    // todo: tracing
    // println!("get_trip: {id:?}");

    use crate::app::auth::utils::get_current_user;
    let current_user = get_current_user()
        .await
        .map_err(|_| ServerFnError::new("Unable to check current user auth"))?;

    match current_user {
        Some(user) => {
            use crate::contexts::use_client;
            let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;

            let trip = Trip::find_by_id(&client, id).await?.expect("No row");
            if trip.owner_id.ne(&user.id) {
                use axum::http::StatusCode;
                let opts = expect_context::<leptos_axum::ResponseOptions>();
                opts.set_status(StatusCode::FORBIDDEN);
                return Err(ServerFnError::new("No access"));
            }
            Ok(trip.into_inner().into())
        }
        None => {
            use axum::http::StatusCode;
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("No current user"))
        }
    }
}
