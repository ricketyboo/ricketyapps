use sqlx::{Pool, Postgres};
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    host: String,
    port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("APP_HOST").expect("Unable to load APP_HOST"),
            port: env::var("APP_PORT")
                .expect("Unable to load APP_PORT")
                .parse::<u16>()
                .expect("unable to convert APP_PORT to int"),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
