use uuid::Uuid;
use welds::WeldsModel;
use super::super::auth::entity::user::UserRow;
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