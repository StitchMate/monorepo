use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PackageQueryError {
    #[error("unknown error")]
    UnknownError,
}
