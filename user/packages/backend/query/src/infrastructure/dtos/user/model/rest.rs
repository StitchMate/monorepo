use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use user_common::{domain::value_object::email::Email, infrastructure::dto::name::rest::RESTName};

use crate::domain::user::model::UserQueryModel;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RESTUserQueryModel {
    pub id: Option<String>,
    pub name: Option<RESTName>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Into<UserQueryModel> for RESTUserQueryModel {
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

impl From<UserQueryModel> for RESTUserQueryModel {
    fn from(value: UserQueryModel) -> Self {
        RESTUserQueryModel {
            id: value.id,
            name: value.name.map(|x| x.into()),
            email: value.email.map(|x| x.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
