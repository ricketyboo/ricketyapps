// use leptos::prelude::*;
// use leptos_router::components::Outlet;
// use rickety::auth::User;
// 
// #[server]
// pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
//     use rickety::state::AppState;
//     use rickety::auth::ssr::UserRecord;
// 
// 
//     match with_context::<AppState, _>(|state| state.pool.clone())
//         .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into())) {
//         Ok(pool) => {
//             let users = UserRecord::get_all(&pool).await;
//             let users_mapped = users.iter().map(User::from).collect();
//             Ok(users_mapped)
//         },
//         Err(e) => Err(e)
//     }
// }
// 
// #[component]
// pub fn AdminView() -> impl IntoView {
//     view! {
//         <h2>Admin</h2>
//         <Outlet />
//     }
// }
// 
// #[component]
// pub fn UserListView() -> impl IntoView {
//     let users_data = OnceResource::new(get_users());
// 
//     view! {
//         <Suspense fallback=|| {
//             view! { <p>Loading...</p> }
//         }>
//             {move || Suspend::new(async move {
//                 if let Ok(users) = users_data.await {
//                     view! {
//                         <For each=move || users.clone() key=|u| u.id let(user)>
//                             <UserListItem user=user />
//                         </For>
//                     }
//                         .into_any()
//                 } else {
// 
//                     view! { <p>"no users"</p> }
//                         .into_any()
//                 }
//             })}
//         </Suspense>
//     }
// }
// 
// #[component]
// fn UserListItem(user: User) -> impl IntoView {
//     use leptos_router::components::A;
//     view! {
//         <p>
//             <A href=user.id.to_string()>{user.username}</A>
//         </p>
//     }
// }
