use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::value_object::name::Name;

#[derive(FromRow, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct SQLName {
    #[sqlx(rename = "first_name")]
    pub first: String,
    #[sqlx(rename = "last_name")]
    pub last: String,
}

impl From<SQLName> for Name {
    fn from(value: SQLName) -> Self {
        Name {
            first: value.first,
            last: value.last,
        }
    }
}

impl From<Name> for SQLName {
    fn from(value: Name) -> Self {
        Self {
            first: value.first,
            last: value.last,
        }
    }
}
