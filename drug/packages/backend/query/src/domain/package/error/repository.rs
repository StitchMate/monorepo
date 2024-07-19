use base::domain::entity::error::RepositoryError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PackageQueryRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Package with {0} {1} does not exist")]
    PackageDoesNotExist(String, String),
    #[error("Package with {0} {1} not found")]
    PackageNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for PackageQueryRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
