use thiserror::Error;

/// Enumeration of possible errors that may occur when dealing with email addresses.
///
/// This enum is used to represent the various possible errors that can occur
/// when dealing with email addresses in the application. Each variant of the enum
/// corresponds to a specific kind of error, and each variant contains a
/// description of the error that can be used to provide more information to
/// the user.
///
#[derive(Error,Debug)]
pub enum EmailError{
    /// The email address is invalid !
    ///
    /// This variant of the enum is used to indicate that the email address is
    /// not in a valid format.
    ///
    #[error("The email address is invalid !")]
    InvalidEmail,
    /// The email address is empty !
    ///
    /// This variant of the enum is used to indicate that the email address is
    /// empty.
    ///
    #[error("The email address is empty !")]
    EmptyEmail
}