pub mod login;
pub mod logout;
pub mod register;

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
