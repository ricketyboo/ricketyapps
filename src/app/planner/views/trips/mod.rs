use leptos::prelude::*;
use crate::app::planner::views::trips::add::AddTrip;
use crate::app::planner::views::trips::list::TripList;

mod add;
mod list;

#[component]
pub fn TripIndex() -> impl IntoView {
    view! {
        <h1>Trips</h1>
        <TripList />
        <AddTrip/>
    }
}

