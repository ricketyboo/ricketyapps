use crate::core::admin::{AdminView, UserListView};
use crate::core::auth::Login;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{ParentRoute, A};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use views::PlacePage;

mod core;
#[cfg(feature = "ssr")]
pub mod ssr;

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

    view! {
        <Stylesheet id="leptos" href="/pkg/planner.css" />

        // sets the document title
        <Title text="Welcome to Planner" />

        // content for this welcome page
        <Router>
            <main>
                <nav>
                    <A href="/">Home</A>
                    <A href="/places">Places</A>
                    // todo: make admin routes only visible if a role check allows them
                    <A href="/admin/users">User Admin</A>
                </nav>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/login") view=Login />
                    <Route path=StaticSegment("/places") view=PlacePage />
                    // todo: make admin routes protected by a role check
                    // https://github.com/leptos-rs/leptos/discussions/2424 vs ProtectedRoute may not work well in an async scenario
                    <ParentRoute path=StaticSegment("/admin") view=AdminView>
                        // todo: add a sub nav view inside admin, and make admin view able to render itself with a "" route
                        <Route path=StaticSegment("users") view=UserListView />
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
        <h1>"Leptos make browser go whrrr "</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
