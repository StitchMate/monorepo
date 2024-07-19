use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum SubsidiariesQueryError {
    #[error("unknown error")]
    UnknownError,
}
