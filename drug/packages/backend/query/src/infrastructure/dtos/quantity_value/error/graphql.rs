use thiserror::Error;

use crate::domain::quantity_value::error::{query::QuantityValueQueryError, repository::QuantityValueQueryRepositoryError, service::QuantityValueQueryServiceError};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLQuantityValueQueryError {
    #[error("A quantityValue with the provided {0} does not exist")]
    QuantityValueNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<QuantityValueQueryServiceError> for GraphQLQuantityValueQueryError {
    fn from(value: QuantityValueQueryServiceError) -> Self {
        match value {
            QuantityValueQueryServiceError::UnknownError => Self::UnknownError,
            QuantityValueQueryServiceError::QuantityValueQueryRepositoryError(e) => match e {
                QuantityValueQueryRepositoryError::QuantityValueNotFound(identifier, value) => {
                    Self::QuantityValueNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            QuantityValueQueryServiceError::QuantityValueQueryError(e) => match e {
                QuantityValueQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
