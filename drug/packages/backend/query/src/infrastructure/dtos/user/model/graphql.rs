use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use user_common::{
    domain::value_object::email::Email, infrastructure::dto::name::graphql::GraphQLName,
};

use crate::domain::user::model::UserQueryModel;

#[derive(SimpleObject, Debug, Default, Clone, PartialEq, Eq)]
pub struct GraphQLUserQueryModel {
    pub id: Option<String>,
    pub name: Option<GraphQLName>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Into<UserQueryModel> for GraphQLUserQueryModel {
    fn into(self) -> UserQueryModel {
        UserQueryModel {
            id: self.id,
            name: self.name.map(|x| x.into()),
            email: self.email.map(|x| Email::new(x)),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<UserQueryModel> for GraphQLUserQueryModel {
    fn from(value: UserQueryModel) -> Self {
        GraphQLUserQueryModel {
            id: value.id,
            name: value.name.map(|x| x.into()),
            email: value.email.map(|x| x.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
