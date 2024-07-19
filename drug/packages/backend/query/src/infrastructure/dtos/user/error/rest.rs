use serde::{Deserialize, Serialize};

use crate::domain::user::error::query::UserQueryError;
use crate::domain::user::error::repository::UserQueryRepositoryError;
use crate::domain::user::error::service::UserQueryServiceError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RESTUserQueryError {
    ApiError(RESTUserError),
    InvalidRequestError(RESTUserError),
}

impl From<UserQueryServiceError> for RESTUserQueryError {
    fn from(value: UserQueryServiceError) -> Self {
        match value {
            UserQueryServiceError::UnknownError => {
                RESTUserQueryError::ApiError(RESTUserError::unknown_error())
            }
            UserQueryServiceError::UserQueryRepositoryError(e) => match e {
                UserQueryRepositoryError::UserDoesNotExist(identifier, value) | UserQueryRepositoryError::UserNotFound(identifier, value) => {
                    RESTUserQueryError::InvalidRequestError(RESTUserError {
                        code: Some("user_not_found".into()),
                        message: Some(
                            format!("A user with the provided {} does not exist", identifier)
                                .into(),
                        ),
                        param_value: Some(value.into()),
                        param: Some(format!("path:{}", identifier)),
                        doc_url: None,
                    })
                }
                _ => RESTUserQueryError::ApiError(RESTUserError::unknown_error()),
            },
            UserQueryServiceError::UserQueryError(e) => match e {
                UserQueryError::UnknownError => {
                    RESTUserQueryError::ApiError(RESTUserError::unknown_error())
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
