use std::sync::Arc;

use axum::async_trait;
use base::{domain::entity::command::VerifyCommand, Command};
use user_common::domain::value_object::{email::Email, name::Name};

use super::error::command::UserCommandError;

#[derive(Command, Debug, Clone, PartialEq)]
pub enum UserCommand {
    CreateUser(CreateUserCommand),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUserCommand {
    pub name: Name,
    pub email: Email,
}

impl Into<UserCommand> for CreateUserCommand {
    fn into(self) -> UserCommand {
        return UserCommand::CreateUser(self);
    }
}

#[async_trait]
impl VerifyCommand for CreateUserCommand {
    type Context = ();
    type Error = UserCommandError;

    async fn verify(&mut self, _context: Arc<Self::Context>) -> Result<(), Vec<Self::Error>> {
        let mut errors = vec![];
        if let Err(e) = self.name.validate() {
            if e.keys().len() > 0 {
                errors.push(UserCommandError::UnknownError);
            }
        }
        if let Err(_e) = self.email.validate() {
            errors.push(UserCommandError::UnknownError)
        }
        if errors.len() > 0 {
            return Err(errors);
        }
        return Ok(());
    }
}
