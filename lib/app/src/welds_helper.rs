use welds::model_traits::hooks::{AfterCreate, AfterUpdate, BeforeCreate, BeforeUpdate};
use welds::model_traits::{ColumnDefaultCheck, HasSchema, UpdateFromRow, WriteToArgs};

pub trait WeldsWriteable:
    HasSchema
    + AfterCreate
    + AfterUpdate
    + BeforeCreate
    + BeforeUpdate
    + ColumnDefaultCheck
    + UpdateFromRow
    + WriteToArgs
{
}

impl<T> WeldsWriteable for T where
    T: HasSchema
        + AfterCreate
        + AfterUpdate
        + BeforeCreate
        + BeforeUpdate
        + ColumnDefaultCheck
        + UpdateFromRow
        + WriteToArgs
{
}
