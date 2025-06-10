// https://github.com/leptos-rs/leptos/blob/main/projects/session_auth_axum/src/state.rs
use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use leptos_axum::AxumRouteListing;
use sqlx::PgPool;

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub routes: Vec<AxumRouteListing>,
}

use secrecy::{ExposeSecret, SecretString};

use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Clone)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseConfiguration {
    pub fn from_env() -> Self {
        Self {
            username: env::var("DB_USERNAME").expect("Unable to load DB_USERNAME"),
            password: env::var("DB_PASSWORD")
                .expect("Unable to load DB_PASSWORD")
                .into(),
            host: env::var("DB_HOST").expect("Unable to load DB_HOST"),
            port: env::var("DB_PORT")
                .expect("Unable to load DB_PORT")
                .parse::<u16>()
                .expect("unable to convert DB_PORT to int"),
            name: env::var("DB_NAME").expect("Unable to load DB_NAME"),
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name,
        )
    }
}

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let db_config = DatabaseConfiguration::from_env();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_config.connection_string())
        .await
}