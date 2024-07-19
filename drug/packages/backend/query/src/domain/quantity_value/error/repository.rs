use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityValueQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("QuantityValue with {0} {1} does not exist")]
    QuantityValueDoesNotExist(String, String),
    #[error("QuantityValue with {0} {1} not found")]
    QuantityValueNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for QuantityValueQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
