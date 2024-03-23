use chrono::{DateTime, Utc};
use struct_field_names_as_array::FieldNamesAsArray;
use user_common::domain::value_object::{email::Email, name::Name};

#[derive(Clone, Debug, FieldNamesAsArray, Default)]
pub struct UserQueryModel {
    pub id: Option<String>,
    pub name: Option<Name>,
    pub email: Option<Email>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}