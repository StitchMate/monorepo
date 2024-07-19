use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PreciseIngredientQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("PreciseIngredient with {0} {1} does not exist")]
    PreciseIngredientDoesNotExist(String, String),
    #[error("PreciseIngredient with {0} {1} not found")]
    PreciseIngredientNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for PreciseIngredientQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
