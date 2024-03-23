use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use user_common::infrastructure::dto::name::rest::RESTName;

use crate::domain::user::aggregate::UserAggregate;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RESTUserCommandModel {
    pub id: Option<String>,
    pub name: Option<RESTName>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserAggregate> for RESTUserCommandModel {
    fn from(value: UserAggregate) -> Self {
        RESTUserCommandModel {
            id: value.id,
            name: value.name.map(|x| x.into()),
            email: value.email.map(|x| x.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
