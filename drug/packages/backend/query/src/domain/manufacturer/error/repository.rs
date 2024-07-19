use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ManufacturerQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Manufacturer with {0} {1} does not exist")]
    ManufacturerDoesNotExist(String, String),
    #[error("Manufacturer with {0} {1} not found")]
    ManufacturerNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for ManufacturerQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
