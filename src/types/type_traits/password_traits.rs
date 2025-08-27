use std::fmt::{Display, Formatter};
use super::super::{password::Password, type_errors::password_error::PasswordError};
impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash_str())
    }
}

impl TryInto<Password> for &str {
    type Error = PasswordError;
    fn try_into(self) -> Result<Password, Self::Error> {
        Password::new(self)
    }
}

impl TryInto<Password> for String {
    type Error = PasswordError;

    fn try_into(self) -> Result<Password, Self::Error> {
        Password::new(self.as_str())
    }
}