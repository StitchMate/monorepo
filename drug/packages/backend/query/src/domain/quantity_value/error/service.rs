use thiserror::Error;

use super::{query::QuantityValueQueryError, repository::QuantityValueQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityValueQueryServiceError {
    #[error("repository error: {0}")]
    QuantityValueQueryRepositoryError(QuantityValueQueryRepositoryError),
    #[error("command handle error: {0}")]
    QuantityValueQueryError(QuantityValueQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<QuantityValueQueryRepositoryError> for QuantityValueQueryServiceError {
    fn from(value: QuantityValueQueryRepositoryError) -> Self {
        Self::QuantityValueQueryRepositoryError(value)
    }
}

impl From<QuantityValueQueryError> for QuantityValueQueryServiceError {
    fn from(value: QuantityValueQueryError) -> Self {
        Self::QuantityValueQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityValueProjectorServiceError {
    #[error("repository error: {0}")]
    QuantityValueQueryRepositoryError(QuantityValueQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<QuantityValueQueryRepositoryError> for QuantityValueProjectorServiceError {
    fn from(value: QuantityValueQueryRepositoryError) -> Self {
        Self::QuantityValueQueryRepositoryError(value)
    }
}