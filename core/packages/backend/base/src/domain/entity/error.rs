use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CommandError {
    #[error("invalid command: {0}")]
    InvalidCommand(String),
    #[error("unknown error occured")]
    UnknownError,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QueryError {
    #[error("invalid query: {0}")]
    InvalidQuery(String),
    #[error("unknown error occured")]
    UnknownError,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LockManagerError {
    #[error("failed to determine existence: {0}")]
    ExistsError(String),
    #[error("failed to unlock: {0}")]
    UnlockError(String),
    #[error("failed to lock: {0}")]
    LockError(String),
    #[error("failed to connect: {0}")]
    ConnectionError(String),
    #[error("unknown error occured")]
    UnknownError,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RepositoryError {
    #[error("authentication failed: {0}")]
    AuthenticationError(String),
    #[error("connection failed: {0}")]
    ConnectionError(String),
    #[error("transaction failed: {0}")]
    TransactionError(String),
    #[error("failed to retrieve: {0}")]
    RetrieveError(String),
    #[error("failed to persist: {0}")]
    PersistError(String),
    #[error("failed to delete: {0}")]
    DeleteError(String),
    #[error("failed to perform migration: {0}")]
    MigrationError(String),
    #[error("unknown error occured")]
    UnknownError,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum EventBusError {
    #[error("failed to authenticate: {0}")]
    AuthenticationError(String),
    #[error("failed to send event: {0}")]
    SendError(String),
    #[error("failed to retrieve events: {0}")]
    RetrieveError(String),
    #[error("failed to connect: {0}")]
    ConnectionError(String),
    #[error("unknown error occured")]
    UnknownError,
}
