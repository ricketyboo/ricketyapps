use axum::Router;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::SessionPgPool;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_app::ssr::state::AppState;
use leptos_app::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    use leptos_app::auth::*;

    let client = core_libs::db::get_client().await;
    let pool = client.as_sqlx_pool();
    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .expect("Unable to run migrations");

    let session_config = SessionConfig::default().with_table_name("sessions");
    // todo: redis sessions instead of pg
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(SessionPgPool::from(pool.clone())), session_config)
            .await
            .expect("Unable to initialise sessions");
    let auth_config = AuthConfig::<Uuid>::default();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        client: client.clone(),
        routes: routes.clone(),
    };

    let app = Router::new()
        .leptos_routes(&app_state, routes, {
            let leptos_options = app_state.leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(
            AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
