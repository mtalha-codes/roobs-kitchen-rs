use super::super::email::Email;
use std::fmt::Display;
use crate::types::type_errors::email_error::EmailError;

/// Implementation of the `Display` trait for `Email`.
///
/// This allows `Email` to be formatted using the `format!` macro,
/// or interpolated into a string using `{}`.
impl Display for Email {
    /// Formats the value using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// The formatted value.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email_str())
    }
}

/// Implementation of the `TryInto<Email>` trait for `&str`.
///
/// This allows a `&str` to be converted into an `Email` using the `try_into` function.
impl TryInto<Email> for &str {
    /// The type returned in the event of a conversion error.
    type Error = EmailError;

    /// Converts the value of `self` into an `Email`.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn try_into(self) -> Result<Email, Self::Error> {
        Email::new(self)
    }
}

/// Implementation of the `TryInto<Email>` trait for `String`.
///
/// This allows a `String` to be converted into an `Email` using the `try_into` function.
impl TryInto<Email> for String {
    /// The type returned in the event of a conversion error.
    type Error = EmailError;

    /// Converts the value of `self` into an `Email`.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn try_into(self) -> Result<Email, Self::Error> {
        Email::new(self.as_str())
    }
}