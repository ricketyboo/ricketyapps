use super::entity::{CreateEntity, CreateInput, CreateResult, DetailView, Entity, ListItem};
use app::{AppState, create_handler};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;
use welds::prelude::{DbState, VecStateExt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_all).post(create_handler::<CreateEntity, CreateInput, CreateResult>),
        )
        .route("/{id}", get(get_one))
}

#[axum::debug_handler]
async fn get_all(State(state): State<AppState>) -> (StatusCode, Json<Vec<ListItem>>) {
    let results = Entity::all().run(&state.client).await;
    let list = results
        .unwrap()
        .into_inners()
        .into_iter()
        .map(ListItem::from)
        .collect();
    (StatusCode::OK, Json(list))
}

#[axum::debug_handler]
async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<DetailView>) {
    let result = Entity::find_by_id(&state.client, id).await;
    (
        StatusCode::OK,
        Json(DetailView::from(result.unwrap().unwrap().into_inner())),
    )
}
