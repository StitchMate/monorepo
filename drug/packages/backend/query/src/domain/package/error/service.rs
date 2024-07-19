use thiserror::Error;

use super::{query::PackageQueryError, repository::PackageQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PackageQueryServiceError {
    #[error("repository error: {0}")]
    PackageQueryRepositoryError(PackageQueryRepositoryError),
    #[error("command handle error: {0}")]
    PackageQueryError(PackageQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<PackageQueryRepositoryError> for PackageQueryServiceError {
    fn from(value: PackageQueryRepositoryError) -> Self {
        Self::PackageQueryRepositoryError(value)
    }
}

impl From<PackageQueryError> for PackageQueryServiceError {
    fn from(value: PackageQueryError) -> Self {
        Self::PackageQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PackageProjectorServiceError {
    #[error("repository error: {0}")]
    PackageQueryRepositoryError(PackageQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<PackageQueryRepositoryError> for PackageProjectorServiceError {
    fn from(value: PackageQueryRepositoryError) -> Self {
        Self::PackageQueryRepositoryError(value)
    }
}