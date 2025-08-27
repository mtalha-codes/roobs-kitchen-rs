use thiserror::Error;

#[derive(Error,Debug)]
pub enum PasswordError {
    #[error("The password is invalid because {0}.")]
    InvalidPassword(String),
    #[error("The password cannot be hashed due to an internal error.")]
    HashingFailed,
    #[error("The hashed password cannot be parsed for verification.")]
    HashParsingFailed,
    #[error("The password does not match !")]
    WrongPassword,
}