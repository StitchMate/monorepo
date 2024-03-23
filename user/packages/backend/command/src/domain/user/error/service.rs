use thiserror::Error;

use super::{command::UserCommandError, repository::UserEventRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserCommandServiceError {
    #[error("repository error: {0}")]
    UserEventRepositoryError(UserEventRepositoryError),
    #[error("command handle error: {0}")]
    UserCommandError(UserCommandError),
    #[error("unknown error")]
    UnknownError,
}

impl From<UserEventRepositoryError> for UserCommandServiceError {
    fn from(value: UserEventRepositoryError) -> Self {
        Self::UserEventRepositoryError(value)
    }
}

impl From<UserCommandError> for UserCommandServiceError {
    fn from(value: UserCommandError) -> Self {
        Self::UserCommandError(value)
    }
}
