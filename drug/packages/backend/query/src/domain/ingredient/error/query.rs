use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum IngredientQueryError {
    #[error("unknown error")]
    UnknownError,
}
