use thiserror::Error;

use crate::domain::user::error::{command::UserCommandError, service::UserCommandServiceError};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLUserCommandError {
    #[error("The provided email address is already in use by another account")]
    EmailAddressInUse(String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<UserCommandServiceError> for GraphQLUserCommandError {
    fn from(value: UserCommandServiceError) -> Self {
        match value {
            UserCommandServiceError::UnknownError
            | UserCommandServiceError::UserEventRepositoryError(_) => Self::UnknownError,
            UserCommandServiceError::UserCommandError(e) => match e {
                UserCommandError::UserAlreadyExists(_, value) => {
                    GraphQLUserCommandError::EmailAddressInUse(value)
                }
                UserCommandError::UnknownError => Self::UnknownError,
            },
        }
    }
}
