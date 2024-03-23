use async_graphql::{ComplexObject, SimpleObject};

use crate::domain::value_object::name::Name;

#[derive(SimpleObject, Debug, Default, Clone, PartialEq, Eq)]
#[graphql(complex)]
pub struct GraphQLName {
    pub first: String,
    pub last: String,
}

impl From<GraphQLName> for Name {
    fn from(value: GraphQLName) -> Self {
        Name {
            first: value.first,
            last: value.last,
        }
    }
}

impl From<Name> for GraphQLName {
    fn from(value: Name) -> Self {
        Self {
            first: value.first,
            last: value.last,
        }
    }
}

impl GraphQLName {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}

#[ComplexObject]
impl GraphQLName {
    pub async fn full(&self) -> String {
        self.full_name()
    }
}
