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
    println!("create_trip: {payload:?}");

    use crate::contexts::use_client;
    let client = use_client().ok_or_else(|| ServerFnError::new("Server error"))?;

    use crate::app::auth::User;
    use axum_session_auth::AuthSession;
    use axum_session_sqlx::SessionPgPool;
    use sqlx::PgPool;
    use uuid::Uuid;
    let auth = leptos_axum::extract::<AuthSession<User, Uuid, SessionPgPool, PgPool>>().await?;

    if let Some(current_user) = auth.current_user {
        let owner_id = current_user.id;

        use crate::app::planner::entity::Trip;
        let mut model = Trip::new();

        model.name = payload.name;
        model.owner_id = owner_id;

        model.save(&client).await?;

        leptos_axum::redirect(&format!("/trips/{}", model.id));

        return Ok(());
    }
    Err(ServerFnError::new("No user"))
}
