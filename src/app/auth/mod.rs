use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
mod utils;

pub(crate) mod entity;

mod views;

pub mod routes {
    pub use super::views::Login;
    pub use super::views::Register;
    pub use super::views::Logout;

    // #[component(transparent)]
    // pub fn AuthRoutes(logged_in: ReadSignal<bool>) -> impl MatchNestedRoutes + Clone {
    //     view! {
    //         <ParentRoute
    //             path=path!("")
    //             view=|| {
    //                 view! {
    //                     <div id="auth-layout" class="root-layout">
    //                         <p>
    //                             <small>"auth layout"</small>
    //                         </p>
    //                         <Outlet />
    //                         <p>
    //                             <small>"end auth layout"</small>
    //                         </p>
    //                     </div>
    //                 }
    //             }
    //         >
    //             <ProtectedRoute
    //                 path=path!("login")
    //                 condition=move || Some(!logged_in.get())
    //                 redirect_path=|| "/"
    //                 view=Login
    //             />
    //             <ProtectedRoute
    //                 path=path!("register")
    //                 condition=move || Some(!logged_in.get())
    //                 redirect_path=|| "/"
    //                 view=Register
    //             />
    //         </ParentRoute>
    //     }
    //     .into_inner()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub anonymous: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String,
}
