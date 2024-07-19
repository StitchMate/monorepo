use thiserror::Error;

use crate::domain::package::error::{
    query::PackageQueryError, repository::PackageQueryRepositoryError, service::PackageQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLPackageQueryError {
    #[error("A package with the provided {0} does not exist")]
    PackageNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<PackageQueryServiceError> for GraphQLPackageQueryError {
    fn from(value: PackageQueryServiceError) -> Self {
        match value {
            PackageQueryServiceError::UnknownError => Self::UnknownError,
            PackageQueryServiceError::PackageQueryRepositoryError(e) => match e {
                PackageQueryRepositoryError::PackageNotFound(identifier, value) => {
                    Self::PackageNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            PackageQueryServiceError::PackageQueryError(e) => match e {
                PackageQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
