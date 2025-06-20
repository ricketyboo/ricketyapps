use auth::views::login::LoginPage;
use auth::views::logout::LogoutPage;
use auth::views::register::RegisterPage;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::components::{A, Outlet, ParentRoute, Redirect, Route};
use leptos_router::hooks::use_url;
use leptos_router::{
    components::{Router, Routes},
    path,
};
use tasks::views::TaskIndex;

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

#[server(endpoint = "auth/check")]
pub async fn check_auth() -> Result<bool, ServerFnError> {
    auth::session::is_user_logged_in()
        .await
        .map_err(|_| ServerFnError::new("AuthError"))
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
        <Title text="Welcome to Planner" />

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
                                        let is_logged_in = auth_resource
                                            .clone()
                                            .await
                                            .is_ok_and(|r| r);
                                        view! {
                                            <Show
                                                when=move || { is_logged_in }
                                                fallback=move || view! { <Redirect path="/login" /> }
                                            >
                                                <div id="app-layout" class="root-layout">
                                                    <nav id="main-nav">
                                                        <A href="/">"Home"</A>
                                                        <A href="/tasks">"Tasks"</A>

                                                        <A href="/logout">"Logout"</A>
                                                    </nav>
                                                    <Outlet />
                                                </div>
                                            </Show>
                                        }
                                    })}
                                </Suspense>
                            }
                        }
                    >
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("tasks") view=TaskIndex />
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
                            view! {
                                <Suspense fallback=move || {
                                    view! { <p>Loading...</p> }
                                }>
                                    {move || Suspend::new(async move {
                                        let is_logged_in = auth_resource.await.is_ok_and(|r| r);
                                        view! {
                                            <Show
                                                when=move || {
                                                    !is_logged_in
                                                        || navigated().is_some_and(|u| u.eq("/logout"))
                                                }
                                                fallback=move || view! { <Redirect path="/" /> }
                                            >
                                                <div id="auth-layout" class="root-layout">
                                                    <Outlet />
                                                </div>
                                            </Show>
                                        }
                                    })}
                                </Suspense>
                            }
                        }
                    >
                        <Route path=path!("login") view=LoginPage />
                        <Route path=path!("register") view=RegisterPage />
                        <Route path=path!("logout") view=LogoutPage />
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

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    hydrate_body(App);
}
