use thiserror::Error;

use super::{query::ManufacturerQueryError, repository::ManufacturerQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ManufacturerQueryServiceError {
    #[error("repository error: {0}")]
    ManufacturerQueryRepositoryError(ManufacturerQueryRepositoryError),
    #[error("command handle error: {0}")]
    ManufacturerQueryError(ManufacturerQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ManufacturerQueryRepositoryError> for ManufacturerQueryServiceError {
    fn from(value: ManufacturerQueryRepositoryError) -> Self {
        Self::ManufacturerQueryRepositoryError(value)
    }
}

impl From<ManufacturerQueryError> for ManufacturerQueryServiceError {
    fn from(value: ManufacturerQueryError) -> Self {
        Self::ManufacturerQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ManufacturerProjectorServiceError {
    #[error("repository error: {0}")]
    ManufacturerQueryRepositoryError(ManufacturerQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ManufacturerQueryRepositoryError> for ManufacturerProjectorServiceError {
    fn from(value: ManufacturerQueryRepositoryError) -> Self {
        Self::ManufacturerQueryRepositoryError(value)
    }
}