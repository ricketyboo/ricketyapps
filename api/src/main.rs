use dotenvy::dotenv;
use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    init().await
}

async fn init() {
    // note: dotenvy .16+ will change how this works, using EnvLoader
    dotenv().expect(".env file not found");
    let db_config = DatabaseConfiguration::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_config.connection_string())
        .await
        // todo: switch to result/error
        .expect("Failed to create connection pool");
    print!("Successful connection");
}

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
