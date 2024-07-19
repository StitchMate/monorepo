use thiserror::Error;

use super::{query::ProductQueryError, repository::ProductQueryRepositoryError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductQueryServiceError {
    #[error("repository error: {0}")]
    ProductQueryRepositoryError(ProductQueryRepositoryError),
    #[error("command handle error: {0}")]
    ProductQueryError(ProductQueryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ProductQueryRepositoryError> for ProductQueryServiceError {
    fn from(value: ProductQueryRepositoryError) -> Self {
        Self::ProductQueryRepositoryError(value)
    }
}

impl From<ProductQueryError> for ProductQueryServiceError {
    fn from(value: ProductQueryError) -> Self {
        Self::ProductQueryError(value)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductProjectorServiceError {
    #[error("repository error: {0}")]
    ProductQueryRepositoryError(ProductQueryRepositoryError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ProductQueryRepositoryError> for ProductProjectorServiceError {
    fn from(value: ProductQueryRepositoryError) -> Self {
        Self::ProductQueryRepositoryError(value)
    }
}