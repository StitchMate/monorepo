use thiserror::Error;

use crate::domain::precise_ingredient::error::{
    query::PreciseIngredientQueryError, repository::PreciseIngredientQueryRepositoryError, service::PreciseIngredientQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLPreciseIngredientQueryError {
    #[error("A preciseIngredient with the provided {0} does not exist")]
    PreciseIngredientNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<PreciseIngredientQueryServiceError> for GraphQLPreciseIngredientQueryError {
    fn from(value: PreciseIngredientQueryServiceError) -> Self {
        match value {
            PreciseIngredientQueryServiceError::UnknownError => Self::UnknownError,
            PreciseIngredientQueryServiceError::PreciseIngredientQueryRepositoryError(e) => match e {
                PreciseIngredientQueryRepositoryError::PreciseIngredientNotFound(identifier, value) => {
                    Self::PreciseIngredientNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            PreciseIngredientQueryServiceError::PreciseIngredientQueryError(e) => match e {
                PreciseIngredientQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
