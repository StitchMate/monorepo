use async_graphql::InputObject;
use user_common::domain::value_object::{email::Email, name::Name};

use crate::domain::user::command::CreateUserCommand;

#[derive(InputObject, Debug, Default, Clone, PartialEq, Eq)]
pub struct GraphQLCreateUserCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<GraphQLCreateUserCommand> for CreateUserCommand {
    fn from(value: GraphQLCreateUserCommand) -> Self {
        CreateUserCommand {
            email: Email::new(value.email),
            name: Name::new(value.first_name, value.last_name),
        }
    }
}
