use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use utility_types::{Omit, Pick};
use uuid::Uuid;
use welds::WeldsModel;

#[derive(Default, Debug, Pick, Omit, WeldsModel)]
#[pick(
    arg(ident=CreateResult, fields(id,created_at), derive(Debug, PartialEq, Serialize)),
    arg(ident=ListItem, fields(id, username, created_at), derive(Debug, PartialEq, Serialize)),
    forward_attrs(),
)]
#[omit(arg(ident=DetailView, fields(id), derive(Debug, PartialEq, Serialize)), forward_attrs())]
#[welds(table = "users")]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
pub struct User {
    #[welds(primary_key)]
    pub id: Uuid,
    pub username: String,
    // todo: implement SecretString
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) type Entity = User;

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    username: String,
    password: SecretString,
}

impl From<CreateInput> for User {
    fn from(value: CreateInput) -> User {
        let password_hash = format!("transformed!{}", value.password.expose_secret());
        User {
            username: value.username,
            password_hash,
            ..Default::default()
        }
    }
}

fn before_create(model: &mut User) -> welds::errors::Result<()> {
    // https://github.com/weldsorm/welds/issues/122 hopefully I can remove this
    model.created_at = Utc::now();
    Ok(())
}

fn before_update(model: &mut User) -> welds::errors::Result<()> {
    println!("hello?");
    model.updated_at = Utc::now();
    Ok(())
}
