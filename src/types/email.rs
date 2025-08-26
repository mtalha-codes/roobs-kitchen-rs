use email_syntax_verify_opt::ValidateEmail;
use super::type_errors::email_error::EmailError;

#[derive(Debug,PartialEq)]
pub struct Email {
    email: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self,EmailError> {
        if !Self::is_valid(&email) {
            return Err(EmailError::InvalidEmailError);
        }
        if Self::is_empty(&email) {
            return Err(EmailError::EmptyEmailError);
        }
        Ok(Self {
            email
        })
    }
    pub fn email_str(&self) -> &str {
        &self.email
    }
    
    pub fn email_string(&self) ->&String {
        &self.email
    }
    fn is_empty(email: &str) -> bool {
        email.is_empty()
    }
    fn is_valid(email: &str) -> bool {
        email.validate_email()
    }
    
}

