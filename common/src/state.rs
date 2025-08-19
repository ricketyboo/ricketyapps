use axum::extract::FromRef;
use leptos::logging::log;
use leptos::prelude::LeptosOptions;
use leptos_axum::AxumRouteListing;
use std::env;
use welds::connections::postgres::PostgresClient;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub client: PostgresClient,
    pub routes: Vec<AxumRouteListing>,
}

#[derive(Clone)]
pub struct AppSettings {
    pub registration_enabled: bool,
}

impl AppSettings {
    pub fn from_env() -> Self {
        Self {
            registration_enabled: env::var("APP_REGISTRATION_OPEN")
                .unwrap_or_else(|_| {
                    log!("APP_REGISTRATION_OPEN was not found. Disabling.");
                    String::from("false")
                })
                .parse::<bool>()
                .unwrap_or_else(|_| {
                    log!("APP_REGISTRATION_OPEN invalid. Disabling.");
                    false
                }),
        }
    }
}
