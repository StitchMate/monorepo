use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Quantity with {0} {1} does not exist")]
    QuantityDoesNotExist(String, String),
    #[error("Quantity with {0} {1} not found")]
    QuantityNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for QuantityQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
