use thiserror::Error;

use crate::domain::manufacturer::error::{
    query::ManufacturerQueryError, repository::ManufacturerQueryRepositoryError, service::ManufacturerQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLManufacturerQueryError {
    #[error("A manufacturer with the provided {0} does not exist")]
    ManufacturerNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<ManufacturerQueryServiceError> for GraphQLManufacturerQueryError {
    fn from(value: ManufacturerQueryServiceError) -> Self {
        match value {
            ManufacturerQueryServiceError::UnknownError => Self::UnknownError,
            ManufacturerQueryServiceError::ManufacturerQueryRepositoryError(e) => match e {
                ManufacturerQueryRepositoryError::ManufacturerNotFound(identifier, value) => {
                    Self::ManufacturerNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            ManufacturerQueryServiceError::ManufacturerQueryError(e) => match e {
                ManufacturerQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
