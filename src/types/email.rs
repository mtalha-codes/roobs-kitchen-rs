use email_syntax_verify_opt::ValidateEmail;
use super::type_errors::email_error::EmailError;

#[derive(Debug,PartialEq)]
pub struct Email (String);

impl Email {
    pub fn new(email: &str) -> Result<Self,EmailError> {
        if Self::is_empty(email) {
            return Err(EmailError::EmptyEmail);
        }
        if !Self::is_valid(email) {
            return Err(EmailError::InvalidEmail);
        }
        Ok(Self(String::from(email)))
    }
    pub fn email_str(&self) -> &str {
        &self.0
    }

    fn is_empty(email: &str) -> bool {
        email.is_empty()
    }
    fn is_valid(email: &str) -> bool {
        email.validate_email()
    }

}
