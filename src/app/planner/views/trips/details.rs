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
            <ErrorBoundary fallback=|_| view! { "Error" }>
                {move || Suspend::new(async move {
                    match details.await {
                        Ok(trip) => {
                            view! {
                                <p>Details</p>
                                <p>Name: {trip.name}</p>
                            }.into_any()
                        }
                        Err(_) => {
                            view! {
                                <p>Details</p>
                                <p>Unable to load</p>
                            }.into_any()
                        }}
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
    println!("get_trip: {id:?}");
    use crate::contexts::use_client;
    let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;

    use axum_session_auth::AuthSession;
    use axum_session_sqlx::SessionPgPool;
    use sqlx::PgPool;
    use uuid::Uuid;
    let auth = leptos_axum::extract::<AuthSession<User, Uuid, SessionPgPool, PgPool>>().await?;

    use crate::app::auth::User;
    let user = auth.current_user.expect("No active user");

    let trip = Trip::find_by_id(&client, id).await?.expect("No row");
    if trip.owner_id.ne(&user.id) {
        return Err(ServerFnError::new("No access"));
    }

    Ok(trip.into_inner().into())
}
