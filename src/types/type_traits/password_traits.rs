use std::fmt::{Display, Formatter};
use super::super::{password::Password, type_errors::password_error::PasswordError};

/// Implementation of the `Display` trait for `Password`.
///
/// This allows a `Password` to be formatted using the `format!` macro,
/// or interpolated into a string using `{}`.
impl Display for Password {
    /// Formats the value using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// The formatted value.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash_str())
    }
}

/// Implementation of the `TryInto<Password>` trait for `&str`.
///
/// This allows a `&str` to be converted into a `Password` using the `try_into` function.
impl TryInto<Password> for &str {
    /// The type returned in the event of a conversion error.
    type Error = PasswordError;

    /// Converts the value of `self` into a `Password`.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn try_into(self) -> Result<Password, Self::Error> {
        Password::new(self)
    }
}

/// Implementation of the `TryInto<Password>` trait for `String`.
///
/// This allows a `String` to be converted into a `Password` using the `try_into` function.
impl TryInto<Password> for String {
    /// The type returned in the event of a conversion error.
    type Error = PasswordError;

    /// Converts the value of `self` into a `Password`.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn try_into(self) -> Result<Password, Self::Error> {
        Password::new(self.as_str())
    }
}