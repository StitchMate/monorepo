use thiserror::Error;

use super::{query::QuantityQueryError, repository::QuantityQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityQueryServiceError {
    #[error("repository error: {0}")]
    QuantityQueryRepositoryError(QuantityQueryRepositoryError),
    #[error("command handle error: {0}")]
    QuantityQueryError(QuantityQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<QuantityQueryRepositoryError> for QuantityQueryServiceError {
    fn from(value: QuantityQueryRepositoryError) -> Self {
        Self::QuantityQueryRepositoryError(value)
    }
}

impl From<QuantityQueryError> for QuantityQueryServiceError {
    fn from(value: QuantityQueryError) -> Self {
        Self::QuantityQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityProjectorServiceError {
    #[error("repository error: {0}")]
    QuantityQueryRepositoryError(QuantityQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<QuantityQueryRepositoryError> for QuantityProjectorServiceError {
    fn from(value: QuantityQueryRepositoryError) -> Self {
        Self::QuantityQueryRepositoryError(value)
    }
}