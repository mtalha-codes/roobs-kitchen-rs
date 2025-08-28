use thiserror::Error;

/// Enumeration of possible errors that may occur when dealing with passwords.
///
/// This enum is used to represent the various possible errors that can occur
/// when dealing with passwords in the application. Each variant of the enum
/// corresponds to a specific kind of error, and each variant contains a
/// description of the error that can be used to provide more information to
/// the user.
///
#[derive(Error,Debug)]
pub enum PasswordError {
    /// The password is invalid because of a specific reason.
    ///
    /// This variant of the enum is used to indicate that the password is
    /// invalid for a specific reason. The reason for the invalid password is
    /// provided as a string in the error message.
    ///
    #[error("The password is invalid because {0}.")]
    InvalidPassword(String),

    /// The password cannot be hashed due to an internal error.
    ///
    /// This variant of the enum is used to indicate that there was an internal
    /// error when hashing the password.
    ///
    #[error("The password cannot be hashed due to an internal error.")]
    HashingFailed,

    /// The hashed password cannot be parsed for verification.
    ///
    /// This variant of the enum is used to indicate that there was an error
    /// when parsing the hashed password for verification.
    ///
    #[error("The hashed password cannot be parsed for verification.")]
    HashParsingFailed,

    /// The password does not match !
    ///
    /// This variant of the enum is used to indicate that the provided password
    /// does not match the stored hashed password.
    ///
    #[error("The password does not match !")]
    WrongPassword,
}