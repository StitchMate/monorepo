use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum IngredientQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Ingredient with {0} {1} does not exist")]
    IngredientDoesNotExist(String, String),
    #[error("Ingredient with {0} {1} not found")]
    IngredientNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for IngredientQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
