use base::domain::entity::error::{EventBusError, RepositoryError};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserEventRepositoryError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("event bus error: {0}")]
    EventBusError(EventBusError),
    #[error("user with {0} {1} does not exist")]
    UserDoesNotExist(String, String),
    #[error("user with {0} {1} not found")]
    UserNotFound(String, String),
    #[error("unknown error")]
    UnknownError,
}

impl From<RepositoryError> for UserEventRepositoryError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<EventBusError> for UserEventRepositoryError {
    fn from(value: EventBusError) -> Self {
        Self::EventBusError(value)
    }
}
