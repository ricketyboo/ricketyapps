use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CreateTripPayload {
    name: String,
}

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

#[server]
async fn create_trip(payload: CreateTripPayload) -> Result<(), ServerFnError> {
    println!("create_trip: {payload:?}");
    Ok(())
}
