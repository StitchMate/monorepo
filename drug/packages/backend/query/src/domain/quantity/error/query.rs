use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityQueryError {
    #[error("unknown error")]
    UnknownError,
}
