use crate::app::auth::AuthRoutes;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{ProtectedRoute, A};
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

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // todo: read from auth session
    let (logged_in, set_logged_in) = signal(false);

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
                <nav id="main-nav">
                    <Show when=move || logged_in()>
                        <A href="/">"Home"</A>
                        <A href="/places">"Places"</A>
                    </Show>
                    // todo: trigger auth clear
                    <button on:click=move |_| {
                        set_logged_in.update(|n| *n = !*n)
                    }>{move || if logged_in.get() { "Log Out" } else { "Log In" }}</button>
                </nav>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ProtectedRoute
                        path=path!("")
                        condition=move || Some(logged_in.get())
                        redirect_path=|| "/login"
                        view=HomePage
                    />
                    <ProtectedRoute
                        path=path!("places")
                        condition=move || Some(logged_in.get())
                        redirect_path=|| "/login"
                        view=PlacePage
                    />
                    <AuthRoutes logged_in=logged_in />
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
