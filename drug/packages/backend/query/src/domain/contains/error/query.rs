use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContainsQueryError {
    #[error("unknown error")]
    UnknownError,
}
