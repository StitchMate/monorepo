use thiserror::Error;

use crate::domain::user::error::{
    query::UserQueryError, repository::UserQueryRepositoryError, service::UserQueryServiceError,
};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphQLUserQueryError {
    #[error("A user with the provided {0} does not exist")]
    UserNotFound(String, String),
    #[error("An unknown error has occured")]
    UnknownError,
}

impl From<UserQueryServiceError> for GraphQLUserQueryError {
    fn from(value: UserQueryServiceError) -> Self {
        match value {
            UserQueryServiceError::UnknownError => Self::UnknownError,
            UserQueryServiceError::UserQueryRepositoryError(e) => match e {
                UserQueryRepositoryError::UserNotFound(identifier, value) => {
                    Self::UserNotFound(identifier, value)
                }
                _ => Self::UnknownError,
            },
            UserQueryServiceError::UserQueryError(e) => match e {
                UserQueryError::UnknownError => Self::UnknownError,
            },
        }
    }
}
