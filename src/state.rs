use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use leptos_axum::AxumRouteListing;
use welds::connections::postgres::PostgresClient;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub client: PostgresClient,
    pub routes: Vec<AxumRouteListing>,
}
