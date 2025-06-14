

pub struct Place {
    id: Uuid,
    name: String,
}

pub mod views;

#[cfg(feature = "ssr")]
mod entity;
