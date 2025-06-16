use super::super::auth::entity::user::UserRow;
use uuid::Uuid;
use welds::WeldsModel;
#[derive(Debug, WeldsModel)]
#[welds(table = "trips")]
#[welds(BelongsTo(owner, UserRow, "owner_id"))]
pub struct Trip {
    #[welds(primary_key)]
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) owner_id: Uuid,
    pub(crate) start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub(crate) end_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "addresses")]
#[welds(BelongsTo(owner, UserRow, "owner_id"))]
pub struct Address {
    #[welds(primary_key)]
    id: Uuid,
    owner_id: Uuid,
    line1: String,
    line2: Option<String>,
    line3: Option<String>,
    province: Option<String>,
    city: Option<String>,
    postcode: Option<String>,
    country: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "accommodations")]
#[welds(BelongsTo(owner, UserRow, "owner_id"))]
pub struct Accommodation {
    #[welds(primary_key)]
    id: Uuid,
    owner_id: Uuid,
    name: String,
    address_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, WeldsModel)]
#[welds(table = "trip_accommodation_booking")]
pub struct TripAccommodationBooking {
    #[welds(primary_key)]
    id: Uuid,
    trip_id: Uuid,
    accommodation_id: Uuid,
    check_in: chrono::DateTime<chrono::Utc>,
    check_out: chrono::DateTime<chrono::Utc>,
}

// pub struct Destination {
//     id: Uuid,
//     name: String,
// }
//
// pub struct Event {
//     id: Uuid,
//     name: String,
// }
