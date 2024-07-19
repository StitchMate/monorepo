use thiserror::Error;

use crate::domain::contains::error::{query::ContainsQueryError, repository::ContainsQueryRepositoryError, service::ContainsQueryServiceError};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLContainsQueryError {
    #[error("A quantity with the provided {0} does not exist")]
    ContainsNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<ContainsQueryServiceError> for GraphQLContainsQueryError {
    fn from(value: ContainsQueryServiceError) -> Self {
        match value {
            ContainsQueryServiceError::UnknownError => Self::UnknownError,
            ContainsQueryServiceError::ContainsQueryRepositoryError(e) => match e {
                ContainsQueryRepositoryError::ContainsNotFound(identifier, value) => {
                    Self::ContainsNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            ContainsQueryServiceError::ContainsQueryError(e) => match e {
                ContainsQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
