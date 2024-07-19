use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("user with {0} {1} does not exist")]
    UserDoesNotExist(String, String),
    #[error("user with {0} {1} not found")]
    UserNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for UserQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
