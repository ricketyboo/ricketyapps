use crate::{AppState, welds_helper};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use welds::model_traits;
use welds::model_traits::HasSchema;
use welds::prelude::DbState;

pub async fn create_handler<T, I, R>(
    State(state): State<AppState>,
    Json(input): Json<I>,
) -> (StatusCode, Json<R>)
where
    T: welds_helper::WeldsWriteable + From<I>,
    <T as HasSchema>::Schema: model_traits::TableColumns,
    R: From<T>,
{
    let mut model: DbState<T> = DbState::new_uncreated(input.into());
    model.save(&state.client).await.unwrap();
    let result: R = model.into_inner().into();
    (StatusCode::OK, Json(result))
}
