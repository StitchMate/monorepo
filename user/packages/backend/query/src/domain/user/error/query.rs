use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserQueryError {
    #[error("unknown error")]
    UnknownError,
}
