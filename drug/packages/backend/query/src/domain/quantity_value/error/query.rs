use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityValueQueryError {
    #[error("unknown error")]
    UnknownError,
}
