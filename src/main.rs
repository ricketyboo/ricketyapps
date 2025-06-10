use rickety_apps::state::AppState;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rickety_apps::app::*;
    use rickety_apps::db::get_pool;
    use axum_session::{
        SessionConfig,
        SessionLayer,
        SessionStore
    };
    use axum_session_sqlx::SessionPgPool;

    let pool = get_pool().await.expect("Unable to get DB pool");

    let session_config = SessionConfig::default().with_table_name("sessions");
    // todo: redis sessions instead of pg
    let session_store = SessionStore::<SessionPgPool>::new(Some(SessionPgPool::from(pool.clone())), session_config).await.expect("Unable to initialise sessions");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone()
    };

    let app = Router::new()
        .leptos_routes(&app_state, routes, {
            let leptos_options = app_state.leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
