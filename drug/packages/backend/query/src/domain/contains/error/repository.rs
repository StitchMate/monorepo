use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContainsQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Contains with {0} {1} does not exist")]
    ContainsDoesNotExist(String, String),
    #[error("Contains with {0} {1} not found")]
    ContainsNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for ContainsQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
