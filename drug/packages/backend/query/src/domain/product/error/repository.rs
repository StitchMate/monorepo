use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Product with {0} {1} does not exist")]
    ProductDoesNotExist(String, String),
    #[error("Product with {0} {1} not found")]
    ProductNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for ProductQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
