#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};

#[cfg(feature = "ssr")]
pub fn use_pool() -> Option<Pool<Postgres>> {
    use crate::state::AppState;
    use leptos::prelude::with_context;
    with_context::<AppState, _>(|state| state.pool.clone())
}
