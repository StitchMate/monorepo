use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use user_common::{
    domain::value_object::email::Email,
    infrastructure::dto::{name::sql::SQLName, user::event::sql::SQLUserEvent},
};

use crate::domain::user::aggregate::UserAggregate;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SQLUserAggregate {
    pub id: Option<String>,
    pub email: Option<String>,
    pub name: Option<SQLName>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    last_event: Option<SQLUserEvent>,
}

impl From<SQLUserAggregate> for UserAggregate {
    fn from(value: SQLUserAggregate) -> Self {
        UserAggregate {
            id: value.id,
            email: value.email.map(|x| Email::new(x)),
            name: value.name.map(|x| x.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
            last_event: value.last_event.map(|x| x.into()),
            ..Default::default()
        }
    }
}

impl From<UserAggregate> for SQLUserAggregate {
    fn from(value: UserAggregate) -> Self {
        SQLUserAggregate {
            id: value.id,
            email: value.email.map(|x| x.into()),
            name: value.name.map(|x| x.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
            last_event: value.last_event.map(|x| x.into()),
        }
    }
}
