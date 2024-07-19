use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PreciseIngredientQueryError {
    #[error("unknown error")]
    UnknownError,
}
