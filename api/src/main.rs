use app::{AppConfig, AppState};
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router, ServiceExt, debug_handler};
use db::{get_client, get_pool};
use dotenvy::dotenv;

use domains::places::api::routes as place_routes;
use domains::users::api::routes as user_routes;
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

#[tokio::main]
async fn main() {
    let (app_state, app_config) = init().await;
    serve(app_state, app_config).await;
}

async fn init() -> (AppConfig, AppState) {
    // note: dotenvy .16+ will change how this works, using EnvLoader
    dotenv().expect(".env file not found");
    let client = get_client().await;
    (AppConfig::from_env(), AppState::new(client))
}

async fn serve(app_config: AppConfig, app_state: AppState) {
    // let pool = get_pool().await.unwrap();
    // let users = UserRepository::all(&pool).await;
    // println!("Users {:?}", users);

    let api_routes = Router::new()
        .nest("/places", place_routes())
        .nest("/users", user_routes());

    let router = Router::new()
        .nest("/api", api_routes)
        .with_state(app_state.clone());
    // let router = Router::new()
    //     .route("/", get(get_places))
    //     .route("/", post(post_places))
    //     .with_state(app_state.clone());
    let app = NormalizePathLayer::trim_trailing_slash().layer(router);
    //
    let addr = app_config.addr();
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

// async fn get_places(State(state): State<AppState>) -> (StatusCode, Json<Vec<Place>>) {
//     let places = Place::all().run(&state.client).await;
//     (StatusCode::OK, Json(places.unwrap().into_inners()))
// }
//
// #[debug_handler]
// async fn post_places(
//     State(state): State<AppState>,
//     Json(create): Json<CreatePlaceDto>,
// ) -> (StatusCode, Json<Place>) {
//     let mut place = DbState::new_uncreated(Place {
//         id: 0,
//         name: create.name,
//     });
//     place.save(&state.client).await.expect("oh no save failure");
//     (StatusCode::OK, Json(place.into_inner()))
// }
