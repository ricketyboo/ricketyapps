use leptos::prelude::*;
use leptos::reactive::spawn_local;
use crate::app::auth::views::{Login, Register};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Outlet, ParentRoute, Redirect, Route, A};
use leptos_router::hooks::{use_url};
use leptos_router::{
    components::{Router, Routes},
    path,
};

pub mod auth;

mod places;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[server]
pub async fn check_auth() -> Result<bool, ServerFnError> {
    use axum_session_auth::Authentication;
    let auth = leptos_axum::extract::<axum_session_auth::AuthSession<crate::app::auth::User, uuid::Uuid, axum_session_sqlx::SessionPgPool, sqlx::PgPool>>().await?;
    let is_logged_in = auth.current_user.is_some_and(|u| u.is_authenticated());
    Ok(is_logged_in)
    // Err(ServerFnError::new("go away"))
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use axum_session_sqlx::SessionPgPool;
    let auth = leptos_axum::extract::<axum_session_auth::AuthSession<crate::app::auth::User, uuid::Uuid, axum_session_sqlx::SessionPgPool, sqlx::PgPool>>().await?;
    auth.logout_user();
    leptos_axum::redirect("/login");
    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    // todo: try to move this and the effects later into a ProtectedAuthRoute component
    let (navigated, set_navigated) = signal(None::<String>);
    let auth_resource = Resource::new_blocking(navigated, |_| check_auth());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rickety-apps.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                // todo: proper aria labels and structure
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute
                        path=path!("")
                        view=move || {
                            Effect::new(move || {
                                let url = use_url();
                                set_navigated(Some(url().path().to_string()));
                            });
                            view! {
                                <Suspense fallback=move || {
                                    view! { <p>Loading...</p> }
                                }>
                                    {move || Suspend::new(async move {
                                        let is_logged_in = auth_resource.await.is_ok_and(|r| r);
                                        view! {
                                            <Show
                                                when=move || { is_logged_in }
                                                fallback=move || view! { <Redirect path="/login" /> }
                                            >
                                                <div id="app-layout" class="root-layout" style="">
                                                    <p>
                                                        <small>"app layout"{is_logged_in}</small>
                                                    </p>
                                                    <nav id="main-nav">
                                                        <A href="/">"Home"</A>
                                                        <A href="/places">"Places"</A>
                                                        <button on:click=move |_| {
                                                            spawn_local(async {
                                                                logout()
                                                                    .await
                                                                    .expect("Couldn't log out. What does this mean?");
                                                            });
                                                        }>"Logout"</button>
                                                    </nav>
                                                    <Outlet />
                                                    <p>
                                                        <small>"end app layout"</small>
                                                    </p>
                                                </div>
                                            </Show>
                                        }
                                    })}
                                </Suspense>
                            }
                        }
                    >
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("places") view=PlacePage />
                    </ParentRoute>

                    // todo: have to work out how to bring back the transparent routes from auth module, while in this new suspense model
                    // <AuthRoutes logged_in />
                    <ParentRoute
                        path=path!("")
                        view=move || {
                            Effect::new(move || {
                                let url = use_url();
                                set_navigated(Some(url().path().to_string()));
                            });
                            // todo: go back to home if here and logged in already
                            view! { <Outlet /> }
                        }
                    >
                        <Route path=path!("login") view=Login />
                        <Route path=path!("register") view=Register />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn PlacePage() -> impl IntoView {
    view! {
        <h1>"Places!"</h1>
        <p>Todo</p>
    }
}
