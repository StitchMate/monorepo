use thiserror::Error;

use super::{query::IngredientQueryError, repository::IngredientQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum IngredientQueryServiceError {
    #[error("repository error: {0}")]
    IngredientQueryRepositoryError(IngredientQueryRepositoryError),
    #[error("command handle error: {0}")]
    IngredientQueryError(IngredientQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<IngredientQueryRepositoryError> for IngredientQueryServiceError {
    fn from(value: IngredientQueryRepositoryError) -> Self {
        Self::IngredientQueryRepositoryError(value)
    }
}

impl From<IngredientQueryError> for IngredientQueryServiceError {
    fn from(value: IngredientQueryError) -> Self {
        Self::IngredientQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum IngredientProjectorServiceError {
    #[error("repository error: {0}")]
    IngredientQueryRepositoryError(IngredientQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<IngredientQueryRepositoryError> for IngredientProjectorServiceError {
    fn from(value: IngredientQueryRepositoryError) -> Self {
        Self::IngredientQueryRepositoryError(value)
    }
}