use thiserror::Error;

use super::{query::SubsidiariesQueryError, repository::SubsidiariesQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum SubsidiariesQueryServiceError {
    #[error("repository error: {0}")]
    SubsidiariesQueryRepositoryError(SubsidiariesQueryRepositoryError),
    #[error("command handle error: {0}")]
    SubsidiariesQueryError(SubsidiariesQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<SubsidiariesQueryRepositoryError> for SubsidiariesQueryServiceError {
    fn from(value: SubsidiariesQueryRepositoryError) -> Self {
        Self::SubsidiariesQueryRepositoryError(value)
    }
}

impl From<SubsidiariesQueryError> for SubsidiariesQueryServiceError {
    fn from(value: SubsidiariesQueryError) -> Self {
        Self::SubsidiariesQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum SubsidiariesProjectorServiceError {
    #[error("repository error: {0}")]
    SubsidiariesQueryRepositoryError(SubsidiariesQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<SubsidiariesQueryRepositoryError> for SubsidiariesProjectorServiceError {
    fn from(value: SubsidiariesQueryRepositoryError) -> Self {
        Self::SubsidiariesQueryRepositoryError(value)
    }
}