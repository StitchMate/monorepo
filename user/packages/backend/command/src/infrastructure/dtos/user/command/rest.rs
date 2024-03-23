use serde::{Deserialize, Serialize};
use user_common::{domain::value_object::email::Email, infrastructure::dto::name::rest::RESTName};

use crate::domain::user::command::CreateUserCommand;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RESTCreateUserCommand {
    pub name: Option<RESTName>,
    pub email: Option<String>,
}

impl From<RESTCreateUserCommand> for CreateUserCommand {
    fn from(value: RESTCreateUserCommand) -> Self {
        CreateUserCommand {
            email: Email::new(value.email.unwrap_or("".into())),
            name: value.name.unwrap_or(RESTName::default()).into(),
        }
    }
}
