#[cfg(feature = "ssr")]
use welds::connections::postgres::PostgresClient;

#[cfg(feature = "ssr")]
pub fn use_client() -> Option<PostgresClient> {
    use crate::state::AppState;
    use leptos::prelude::with_context;
    with_context::<AppState, _>(|state| state.client.clone())
}
