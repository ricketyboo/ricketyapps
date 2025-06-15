use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub(super) fn AddTrip() -> impl IntoView {
    let action = ServerAction::<CreateTrip>::new();
    view! {
        <p>"Add Trip"</p>
        <ActionForm action>
            <label>"Name"<input name="payload[name]"/></label>
            <button type="submit">Save</button>
        </ActionForm>
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CreateTripPayload {
    name: String,
}

#[server]
async fn create_trip(payload: CreateTripPayload) -> Result<(), ServerFnError> {
    // todo: tracing
    // println!("create_trip: {payload:?}");

    use crate::app::auth::utils::get_current_user;
    let current_user = get_current_user()
        .await
        .map_err(|_| ServerFnError::new("Unable to check current user auth"))?;

    match current_user {
        Some(user) => {
            use crate::contexts::use_client;
            let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;
            
            use crate::app::planner::entity::Trip;
            let mut model = Trip::new();

            model.name = payload.name;
            model.owner_id = user.id;

            model.save(&client).await?;

            leptos_axum::redirect(&format!("/trips/{}", model.id));

            Ok(())
        }
        None => {
            use axum::http::StatusCode;
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("No current user"))
        }
    }
}
