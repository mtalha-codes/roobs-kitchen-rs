use thiserror::Error;

#[derive(Error,Debug)]
pub enum EmailError{
    #[error("The email address is invalid !")]
    InvalidEmail,
    #[error("The email address is empty !")]
    EmptyEmail
}