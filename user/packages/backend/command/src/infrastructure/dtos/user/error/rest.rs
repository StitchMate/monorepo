use serde::{Deserialize, Serialize};

use crate::domain::user::error::{command::UserCommandError, service::UserCommandServiceError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RESTUserCommandError {
    ApiError(RESTUserError),
    InvalidRequestError(RESTUserError),
}

impl From<UserCommandServiceError> for RESTUserCommandError {
    fn from(value: UserCommandServiceError) -> Self {
        match value {
            UserCommandServiceError::UnknownError
            | UserCommandServiceError::UserEventRepositoryError(_) => {
                RESTUserCommandError::ApiError(RESTUserError::unknown_error())
            }
            UserCommandServiceError::UserCommandError(e) => match e {
                UserCommandError::UserAlreadyExists(identifier, value) => {
                    RESTUserCommandError::InvalidRequestError(RESTUserError {
                        code: Some("email_in_use".into()),
                        message: Some(
                            "The provided email address is already used by another account".into(),
                        ),
                        param_value: Some(value.into()),
                        param: Some(format!("payload:{}", identifier)),
                        doc_url: None,
                    })
                }
                UserCommandError::UnknownError => {
                    RESTUserCommandError::ApiError(RESTUserError::unknown_error())
                }
            },
        }
    }
}

//TODO: Move to common
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct RESTUserError {
    pub code: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub param_value: Option<String>,
    pub doc_url: Option<String>,
}

impl RESTUserError {
    pub fn unknown_error() -> Self {
        RESTUserError {
            code: Some("unknown_error".into()),
            message: Some("An unknown error occured".into()),
            ..Default::default()
        }
    }
}
