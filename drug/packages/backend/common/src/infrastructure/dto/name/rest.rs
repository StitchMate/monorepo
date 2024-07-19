use serde::{Deserialize, Serialize};

use crate::domain::value_object::name::Name;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RESTName {
    pub first: String,
    pub last: String,
}

impl From<RESTName> for Name {
    fn from(value: RESTName) -> Self {
        Name {
            first: value.first,
            last: value.last,
        }
    }
}

impl From<Name> for RESTName {
    fn from(value: Name) -> Self {
        Self {
            first: value.first,
            last: value.last,
        }
    }
}
