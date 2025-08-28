use email_syntax_verify_opt::ValidateEmail; // crate for email verification using the trait ValidateEmail, implemented  for &str and String types
use super::type_errors::email_error::EmailError;

#[derive(Debug,PartialEq)]
pub struct Email (String);

impl Email {
    /// Create a new `Email` from a given string, returning an error if
    /// the string does not satisfy the email policy.
    ///
    /// The email policy is as follows:
    ///
    /// - The email address must not be empty.
    /// - The email address must be in a valid format.
    ///
    /// # Errors
    ///
    /// - If the email address is empty, an error of type `EmailError::EmptyEmail` is returned.
    /// - If the email address is invalid, an error of type `EmailError::InvalidEmail` is returned.
    ///
    pub fn new(email: &str) -> Result<Self,EmailError> {
        if Self::is_empty(email) {
            return Err(EmailError::EmptyEmail);
        }
        if !Self::is_valid(email) {
            return Err(EmailError::InvalidEmail);
        }
        Ok(Self(String::from(email)))
    }
    /// Returns the stored email address.
    ///
    /// This is a read-only getter for the stored email address.
    pub fn email_str(&self) -> &str {
        &self.0
    }



    ///
    /// Returns `true` if the given email address is empty, `false` otherwise.
    ///
    fn is_empty(email: &str) -> bool {
        email.is_empty()
    }

    /// Returns `true` if the email address is valid, `false` otherwise.
    ///
    /// An email address is valid if it satisfies the email policy (defined in `email_syntax_verify_opt` crate).
    ///
    fn is_valid(email: &str) -> bool {
        email.validate_email()
    }

}
