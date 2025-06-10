use serde::{Deserialize, Serialize};
use utility_types::Omit;
use welds::WeldsModel;

#[derive(Debug, WeldsModel, Omit, Serialize)]
#[welds(table = "places")]
#[omit(arg(ident = CreatePlaceDto, fields(id), derive(Debug, PartialEq, Deserialize)), forward_attrs())]
pub struct Place {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

impl Place {}
