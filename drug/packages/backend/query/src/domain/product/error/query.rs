use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductQueryError {
    #[error("unknown error")]
    UnknownError,
}
