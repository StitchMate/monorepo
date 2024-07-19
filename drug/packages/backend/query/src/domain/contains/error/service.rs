use thiserror::Error;

use super::{query::ContainsQueryError, repository::ContainsQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContainsQueryServiceError {
    #[error("repository error: {0}")]
    ContainsQueryRepositoryError(ContainsQueryRepositoryError),
    #[error("command handle error: {0}")]
    ContainsQueryError(ContainsQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ContainsQueryRepositoryError> for ContainsQueryServiceError {
    fn from(value: ContainsQueryRepositoryError) -> Self {
        Self::ContainsQueryRepositoryError(value)
    }
}

impl From<ContainsQueryError> for ContainsQueryServiceError {
    fn from(value: ContainsQueryError) -> Self {
        Self::ContainsQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContainsProjectorServiceError {
    #[error("repository error: {0}")]
    ContainsQueryRepositoryError(ContainsQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ContainsQueryRepositoryError> for ContainsProjectorServiceError {
    fn from(value: ContainsQueryRepositoryError) -> Self {
        Self::ContainsQueryRepositoryError(value)
    }
}