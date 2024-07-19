use thiserror::Error;

use crate::domain::quantity::error::{
    query::QuantityQueryError, repository::QuantityQueryRepositoryError, service::QuantityQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLQuantityQueryError {
    #[error("A quantity with the provided {0} does not exist")]
    QuantityNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<QuantityQueryServiceError> for GraphQLQuantityQueryError {
    fn from(value: QuantityQueryServiceError) -> Self {
        match value {
            QuantityQueryServiceError::UnknownError => Self::UnknownError,
            QuantityQueryServiceError::QuantityQueryRepositoryError(e) => match e {
                QuantityQueryRepositoryError::QuantityNotFound(identifier, value) => {
                    Self::QuantityNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            QuantityQueryServiceError::QuantityQueryError(e) => match e {
                QuantityQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
