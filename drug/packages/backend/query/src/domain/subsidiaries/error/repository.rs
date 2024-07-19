use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum SubsidiariesQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("subsidiaries with {0} {1} does not exist")]
    SubsidiariesDoesNotExist(String, String),
    #[error("subsidiaries with {0} {1} not found")]
    SubsidiariesNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for SubsidiariesQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
