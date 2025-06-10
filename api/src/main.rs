use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router, ServiceExt};
use db::get_pool;
use dotenvy::dotenv;
use models::Place;
use sqlx::{Pool, Postgres};
use std::env;
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

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

#[tokio::main]
async fn main() {
    let (app_state, app_config) = init().await;
    serve(app_state, app_config).await;
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

async fn init() -> (AppConfig, AppState) {
    // note: dotenvy .16+ will change how this works, using EnvLoader
    dotenv().expect(".env file not found");
    let pool = get_pool().await.expect("Oh no, pool is dead.");
    let app_config = AppConfig::from_env();
    (app_config, AppState::new(pool))
}

async fn serve(app_config: AppConfig, app_state: AppState) {
    let router = Router::new()
        .route("/", get(get_places))
        .with_state(app_state.clone());
    let app = NormalizePathLayer::trim_trailing_slash().layer(router);

    let addr = app_config.addr();
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

async fn get_places(State(state): State<AppState>) -> (StatusCode, Json<Vec<Place>>) {
    // async fn what() -> (StatusCode, Json<Vec<&'static str>>) {
    let places = sqlx::query_as!(models::Place, "SELECT id, name FROM places")
        .fetch_all(&state.pool)
        .await;
    (StatusCode::OK, Json(places.unwrap()))
}
