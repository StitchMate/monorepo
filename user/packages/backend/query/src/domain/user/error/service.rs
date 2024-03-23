use thiserror::Error;

use super::{query::UserQueryError, repository::UserQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserQueryServiceError {
    #[error("repository error: {0}")]
    UserQueryRepositoryError(UserQueryRepositoryError),
    #[error("command handle error: {0}")]
    UserQueryError(UserQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<UserQueryRepositoryError> for UserQueryServiceError {
    fn from(value: UserQueryRepositoryError) -> Self {
        Self::UserQueryRepositoryError(value)
    }
}

impl From<UserQueryError> for UserQueryServiceError {
    fn from(value: UserQueryError) -> Self {
        Self::UserQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserProjectorServiceError {
    #[error("repository error: {0}")]
    UserQueryRepositoryError(UserQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<UserQueryRepositoryError> for UserProjectorServiceError {
    fn from(value: UserQueryRepositoryError) -> Self {
        Self::UserQueryRepositoryError(value)
    }
}