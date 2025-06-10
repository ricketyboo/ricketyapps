use leptos::prelude::with_context;
use sqlx::{Pool, Postgres};
use crate::state::AppState;

#[cfg(feature = "ssr")]
pub fn use_pool() -> Option<Pool<Postgres>> {
    with_context::<AppState, _>(|state| state.pool.clone())
}
