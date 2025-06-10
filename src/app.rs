
use leptos::prelude::*;

use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Outlet, ParentRoute,  Redirect, Route, A};
use leptos_router::{
    components::{Router, Routes},
    path,
};
use leptos_router::hooks::{use_navigate, use_url};
use crate::app::auth::views::Login;

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
async fn check_auth() -> Result<bool, ServerFnError>{
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    // Ok(true)
    // leptos_axum::redirect("/login");
    Ok(false)
    // Err(ServerFnError::new("go away"))
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // todo: read from auth session
    let (logged_in, set_logged_in) = signal(true);
    
    
    

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
                            // having to do this here otherwise we get errors about not being in a router context.
                            // todo: see if I can move this whole chunk of madness into an AuthenticatedRoute

                            let url = use_url();
                            let auth = Resource::new(url, move |_| check_auth());
                            Effect::new(move |_| {
                                let is_logged_in = auth
                                    .get()
                                    .is_some_and(|res| {
                                        res.is_ok_and(|is_logged_in| is_logged_in)
                                    });
                                if !is_logged_in {
                                    let navigate = use_navigate();
                                    navigate("/login", Default::default());
                                }
                            });
                            view! {
                                <Suspense fallback=|| {
                                    view! { <p>"Loading \"/protected\"..."</p> }
                                }>
                                    <Show when=move || {
                                        auth.get()
                                            .is_some_and(|res| {
                                                res.is_ok_and(|is_logged_in| is_logged_in)
                                            })
                                    }>
                                        <div id="app-layout" class="root-layout" style="">
                                            <p>
                                                <small>"app layout"</small>
                                            </p>
                                            <nav id="main-nav">
                                                <A href="/">"Home"</A>
                                                <A href="/places">"Places"</A>
                                            </nav>
                                            <Outlet />
                                            <p>
                                                <small>"end app layout"</small>
                                            </p>
                                        </div>
                                    </Show>
                                </Suspense>
                            }
                        }
                    >
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("places") view=PlacePage />
                    </ParentRoute>
                    <Route path=path!("login") view=Login />

                // <AuthRoutes logged_in />
                </Routes>
            </main>
            // todo: trigger auth clear
            <button on:click=move |_| {
                set_logged_in.update(|n| *n = !*n)
            }>{move || if logged_in.get() { "Log Out" } else { "Log In" }}</button>
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
