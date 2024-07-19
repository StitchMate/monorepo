use thiserror::Error;

use crate::domain::ingredient::error::{
    query::IngredientQueryError, repository::IngredientQueryRepositoryError, service::IngredientQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLIngredientQueryError {
    #[error("A ingredient with the provided {0} does not exist")]
    IngredientNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<IngredientQueryServiceError> for GraphQLIngredientQueryError {
    fn from(value: IngredientQueryServiceError) -> Self {
        match value {
            IngredientQueryServiceError::UnknownError => Self::UnknownError,
            IngredientQueryServiceError::IngredientQueryRepositoryError(e) => match e {
                IngredientQueryRepositoryError::IngredientNotFound(identifier, value) => {
                    Self::IngredientNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            IngredientQueryServiceError::IngredientQueryError(e) => match e {
                IngredientQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
