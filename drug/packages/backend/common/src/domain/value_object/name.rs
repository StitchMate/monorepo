use std::collections::BTreeMap;

use crate::domain::error::name::NameError;

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Name {
    pub first: String,
    pub last: String
}

impl Name {
    pub fn validate(&self) -> Result<(), BTreeMap<String, Vec<NameError>>> {
        let mut errors = BTreeMap::new();
        if self.first.chars().count() == 0 {
            errors.insert("first".into(), vec![NameError::FirstNameTooShort]);
        }
        if self.last.chars().count() == 0 {
            errors.insert("last".into(), vec![NameError::LastNameTooShort]);
        }
        if errors.is_empty() {
            return Err(errors)
        }
        Ok(())
    }
}

impl Name {
    pub fn new<S: Into<String>>(first: S, last: S) -> Self {
        Self {
            first: first.into(),
            last: last.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_valid_name() {
        //Arrange
        let name = Name::new("john", "smith");
        //Act
        let validation_result = name.validate();
        //Assert
        assert_eq!(validation_result, Ok(()));
    }

    #[test]
    fn it_reports_error_when_no_name_present() {
        //Arrange
        let name: Name = Name::new("", "");
        let mut expected = BTreeMap::new();
        expected.insert("first".into(), vec![NameError::FirstNameTooShort]);
        expected.insert("last".into(), vec![NameError::LastNameTooShort]);
        //Act
        let validation_result = name.validate();
        //Assert
        assert_eq!(validation_result, Err(expected));
    }

    #[test]
    fn it_reports_error_when_no_first_name_present() {
        //Arrange
        let name: Name = Name::new("", "smith");
        let mut expected = BTreeMap::new();
        expected.insert("first".into(), vec![NameError::FirstNameTooShort]);
        //Act
        let validation_result = name.validate();
        //Assert
        assert_eq!(validation_result, Err(expected));
    }

    #[test]
    fn it_reports_error_when_no_last_name_present() {
        //Arrange
        let name: Name = Name::new("john", "");
        let mut expected = BTreeMap::new();
        expected.insert("last".into(), vec![NameError::LastNameTooShort]);
        //Act
        let validation_result = name.validate();
        //Assert
        assert_eq!(validation_result, Err(expected));
    }
}