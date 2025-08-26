use thiserror::Error;
#[derive(Error,Debug)]
pub enum EmailError{
    #[error("The email address is invalid !")]
    InvalidEmailError,
    #[error("The email address is empty !")]
    EmptyEmailError
}