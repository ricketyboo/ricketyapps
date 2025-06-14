use leptos::prelude::*;
use crate::app::planner::views::trips::add::AddTrip;

mod add;

#[component]
pub fn TripIndex() -> impl IntoView {
    view! {
        <p>Trips</p>
        <AddTrip/>
    }
}

