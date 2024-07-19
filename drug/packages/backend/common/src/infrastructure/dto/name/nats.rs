use serde::{Deserialize, Serialize};

use crate::domain::value_object::name::Name;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct NATSName {
    pub first: String,
    pub last: String,
}

impl From<NATSName> for Name {
    fn from(value: NATSName) -> Self {
        Name {
            first: value.first,
            last: value.last,
        }
    }
}

impl From<Name> for NATSName {
    fn from(value: Name) -> Self {
        Self {
            first: value.first,
            last: value.last,
        }
    }
}
