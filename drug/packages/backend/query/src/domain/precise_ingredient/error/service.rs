use thiserror::Error;

use super::{query::PreciseIngredientQueryError, repository::PreciseIngredientQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PreciseIngredientQueryServiceError {
    #[error("repository error: {0}")]
    PreciseIngredientQueryRepositoryError(PreciseIngredientQueryRepositoryError),
    #[error("command handle error: {0}")]
    PreciseIngredientQueryError(PreciseIngredientQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<PreciseIngredientQueryRepositoryError> for PreciseIngredientQueryServiceError {
    fn from(value: PreciseIngredientQueryRepositoryError) -> Self {
        Self::PreciseIngredientQueryRepositoryError(value)
    }
}

impl From<PreciseIngredientQueryError> for PreciseIngredientQueryServiceError {
    fn from(value: PreciseIngredientQueryError) -> Self {
        Self::PreciseIngredientQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PreciseIngredientProjectorServiceError {
    #[error("repository error: {0}")]
    PreciseIngredientQueryRepositoryError(PreciseIngredientQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<PreciseIngredientQueryRepositoryError> for PreciseIngredientProjectorServiceError {
    fn from(value: PreciseIngredientQueryRepositoryError) -> Self {
        Self::PreciseIngredientQueryRepositoryError(value)
    }
}