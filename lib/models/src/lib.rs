use serde::Serialize;

#[derive(Serialize)]
pub struct Place {
    pub id: i32,
    pub name: String,
}
