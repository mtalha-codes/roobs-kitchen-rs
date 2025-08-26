use super::super::email::Email;
use std::fmt::Display;
use crate::types::type_errors::email_error::EmailError;

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email_str())
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value.into())
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}


