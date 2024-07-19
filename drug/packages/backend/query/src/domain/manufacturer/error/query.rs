use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ManufacturerQueryError {
    #[error("unknown error")]
    UnknownError,
}
