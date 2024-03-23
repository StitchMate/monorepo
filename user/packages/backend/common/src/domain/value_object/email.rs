use std::fmt::Display;

use validator::validate_email;

use crate::domain::error::email::EmailError;

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Email {
    value: String
}

impl Email {
    pub fn validate(&self) -> Result<(), EmailError> {
        if self.value.chars().count() == 0 {
            return Err(EmailError::EmailTooShort)
        }
        if !validate_email(&self.value) {
            return Err(EmailError::EmailInvalid)
        }
        Ok(())
    }
}

impl Email {
    pub fn new<S: Into<String>>(email: S) -> Self {
        Self {
            value: email.into()
        }
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.value
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_valid_email() {
        //Arrange
        let email = Email::new("test@stitchmate.io");
        //Act
        let validation_result = email.validate();
        //Assert
        assert_eq!(validation_result, Ok(()));
    }

    #[test]
    fn it_reports_error_when_no_email_present() {
        //Arrange
        let email = Email::new("");
        //Act
        let validation_result = email.validate();
        //Assert
        assert_eq!(validation_result, Err(EmailError::EmailTooShort));
    }

    #[test]
    fn it_reports_error_when_email_invalid() {
        //Arrange
        let email = Email::new("invalid-email");
        //Act
        let validation_result = email.validate();
        //Assert
        assert_eq!(validation_result, Err(EmailError::EmailInvalid));
    }
}