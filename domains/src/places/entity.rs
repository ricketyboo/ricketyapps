use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utility_types::{Omit, Pick};
use uuid::Uuid;
use welds::WeldsModel;

#[derive(Default, Debug, Omit, Pick, WeldsModel)]
#[pick(
    arg(ident=CreateInput, fields(name), derive(Debug, PartialEq, Deserialize)),
    arg(ident=CreateResult, fields(id,created_at), derive(Debug, PartialEq, Serialize)),
    arg(ident=ListItem, fields(id, name, created_at), derive(Debug, PartialEq, Serialize)),
    forward_attrs(),
)]
#[omit(arg(ident=DetailView, fields(id), derive(Debug, PartialEq, Serialize)), forward_attrs())]
#[welds(table = "places")]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
pub struct Place {
    #[welds(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) type Entity = Place;

impl From<CreateInput> for Place {
    fn from(value: CreateInput) -> Place {
        Place {
            name: value.name,
            ..Default::default()
        }
    }
}

fn before_create(model: &mut Place) -> welds::errors::Result<()> {
    // https://github.com/weldsorm/welds/issues/122 hopefully I can remove this
    model.created_at = Utc::now();
    Ok(())
}

fn before_update(model: &mut Place) -> welds::errors::Result<()> {
    model.updated_at = Utc::now();
    Ok(())
}
