use chrono::{DateTime, Utc};
use sqlx::FromRow;
use user_common::{
    domain::value_object::{email::Email, name::Name},
    infrastructure::dto::name::sql::SQLName,
};

use crate::domain::user::model::UserQueryModel;

#[derive(FromRow, Debug, Default, Clone)]
pub struct SQLUserQueryModel {
    #[sqlx(default)]
    pub id: Option<String>,
    #[sqlx(flatten)]
    pub name: SQLName,
    #[sqlx(default)]
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Into<UserQueryModel> for SQLUserQueryModel {
    fn into(self) -> UserQueryModel {
        let mut name: Option<Name> = None;
        if self.name.first.len() > 0 && self.name.last.len() > 0 {
            name = Some(self.name.into());
        }
        UserQueryModel {
            id: self.id,
            name: name,
            email: self.email.map(|x| Email::new(x)),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}
