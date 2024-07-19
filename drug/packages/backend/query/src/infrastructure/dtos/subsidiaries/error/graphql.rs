use thiserror::Error;

use crate::domain::subsidiaries::error::{
    query::SubsidiariesQueryError, repository::SubsidiariesQueryRepositoryError, service::SubsidiariesQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLSubsidiariesQueryError {
    #[error("A subsidiaries with the provided {0} does not exist")]
    SubsidiariesNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<SubsidiariesQueryServiceError> for GraphQLSubsidiariesQueryError {
    fn from(value: SubsidiariesQueryServiceError) -> Self {
        match value {
            SubsidiariesQueryServiceError::UnknownError => Self::UnknownError,
            SubsidiariesQueryServiceError::SubsidiariesQueryRepositoryError(e) => match e {
                SubsidiariesQueryRepositoryError::SubsidiariesNotFound(identifier, value) => {
                    Self::SubsidiariesNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            SubsidiariesQueryServiceError::SubsidiariesQueryError(e) => match e {
                SubsidiariesQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
