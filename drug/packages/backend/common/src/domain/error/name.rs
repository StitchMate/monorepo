use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum NameError {
    #[error("first name must be atleast 1 character")]
    FirstNameTooShort,
    #[error("last name must be atleast 1 character")]
    LastNameTooShort,
}