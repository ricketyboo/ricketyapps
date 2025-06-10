use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utility_types::{Omit, Partial, Pick};
use uuid::Uuid;
use welds::WeldsModel;

#[derive(Default, Debug, Omit, Pick, Partial, WeldsModel)]
#[pick(
    arg(ident=CreateInput, fields(name), derive(Debug, PartialEq, Deserialize, WeldsModel), forward_attrs(welds)),
    arg(ident=CreateEntity, fields(id,name), derive(Debug, PartialEq, Serialize, Deserialize,WeldsModel), forward_attrs(welds)),
    arg(ident=CreateResult, fields(id), derive(Debug, PartialEq, Serialize), forward_attrs()),
    arg(ident=ListItem, fields(id, name, created_at,updated_at), derive(Debug, PartialEq, Serialize), forward_attrs()),
)]
#[partial(ident=DetailView, derive(Debug, PartialEq, Serialize), forward_attrs())]
#[welds(table = "places")]
pub struct Place {
    #[welds(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub(crate) type Entity = Place;

impl From<CreateInput> for CreateEntity {
    fn from(value: CreateInput) -> Self {
        Self {
            id: Default::default(),
            name: value.name,
        }
    }
}
impl From<CreateEntity> for CreateResult {
    fn from(value: CreateEntity) -> Self {
        Self { id: value.id }
    }
}
