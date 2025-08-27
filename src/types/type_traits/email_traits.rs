use super::super::email::Email;
use std::fmt::Display;
use crate::types::type_errors::email_error::EmailError;

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email_str())
    }
}


impl TryInto<Email> for &str {
    type Error = EmailError;
    fn try_into(self) -> Result<Email, Self::Error> {
        Email::new(self)
    }
}

impl TryInto<Email> for String {
    type Error = EmailError;
    fn try_into(self) -> Result<Email, Self::Error> {
        Email::new(self.as_str())
    }
} 
