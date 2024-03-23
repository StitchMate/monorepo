use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserCommandError {
    #[error("user with {0} {1} already exist")]
    UserAlreadyExists(String, String),
    #[error("unknown error")]
    UnknownError,
}
