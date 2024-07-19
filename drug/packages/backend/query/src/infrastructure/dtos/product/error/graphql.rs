use thiserror::Error;

use crate::domain::product::error::{
    query::ProductQueryError, repository::ProductQueryRepositoryError, service::ProductQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLProductQueryError {
    #[error("A product with the provided {0} does not exist")]
    ProductNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<ProductQueryServiceError> for GraphQLProductQueryError {
    fn from(value: ProductQueryServiceError) -> Self {
        match value {
            ProductQueryServiceError::UnknownError => Self::UnknownError,
            ProductQueryServiceError::ProductQueryRepositoryError(e) => match e {
                ProductQueryRepositoryError::ProductNotFound(identifier, value) => {
                    Self::ProductNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            ProductQueryServiceError::ProductQueryError(e) => match e {
                ProductQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
