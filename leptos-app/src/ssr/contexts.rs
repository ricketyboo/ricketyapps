use crate::ssr::state::AppState;
use leptos::prelude::with_context;
use welds::connections::postgres::PostgresClient;

pub fn use_client() -> Option<PostgresClient> {
    with_context::<AppState, _>(|state| state.client.clone())
}
