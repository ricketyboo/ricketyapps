use leptos::prelude::*;
use leptos_router::nested_router::Outlet;
use crate::app::planner::views::trips::add::AddTrip;
use crate::app::planner::views::trips::list::TripList;


mod add;
mod list;

pub mod details;

#[component]
pub fn TripIndex() -> impl IntoView {
    view! {
        <h1>Trips</h1>
        <TripList />
        <AddTrip/>
        <Outlet />
    }
}

