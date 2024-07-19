use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum EmailError {
    #[error("email must be atleast 1 character")]
    EmailTooShort,
    #[error("provided email is invalid")]
    EmailInvalid,
}