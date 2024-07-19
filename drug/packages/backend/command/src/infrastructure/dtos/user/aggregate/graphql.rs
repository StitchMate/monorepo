use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use user_common::infrastructure::dto::name::graphql::GraphQLName;

use crate::domain::user::aggregate::UserAggregate;

#[derive(SimpleObject, Debug, Default, Clone, PartialEq, Eq)]
pub struct GraphQLUserCommandModel {
    pub id: Option<String>,
    pub name: Option<GraphQLName>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserAggregate> for GraphQLUserCommandModel {
    fn from(value: UserAggregate) -> Self {
        GraphQLUserCommandModel {
            id: value.id,
            name: value.name.map(|x| x.into()),
            email: value.email.map(|x| x.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
