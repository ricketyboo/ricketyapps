use std::env;
use welds::connections::any::AnyClient;
use welds::prelude::*;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use hooks::{AfterCreate, AfterUpdate, BeforeCreate, BeforeUpdate};
use model_traits::{ColumnDefaultCheck, UpdateFromRow, WriteToArgs, hooks};
use welds::model_traits;
use welds::model_traits::HasSchema;

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
    pub client: AnyClient,
}

impl AppState {
    pub fn new(client: AnyClient) -> Self {
        Self { client }
    }
}

pub async fn create_handler<T, I, R: From<T>>(
    State(state): State<AppState>,
    Json(input): Json<I>,
) -> (StatusCode, Json<R>)
where
    T: HasSchema
        + AfterCreate
        + AfterUpdate
        + BeforeCreate
        + BeforeUpdate
        + ColumnDefaultCheck
        + UpdateFromRow
        + WriteToArgs
        + From<I>,
    <T as HasSchema>::Schema: model_traits::TableColumns,
{
    let mut model: DbState<T> = DbState::new_uncreated(input.into());
    model.save(&state.client).await.unwrap();
    let result: R = model.into_inner().into();
    (StatusCode::OK, Json(result))
}
