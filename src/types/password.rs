use super::type_errors::password_error::PasswordError;
use argon2::{password_hash::rand_core::OsRng, password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use pval::Pval;

#[derive(Debug)]
pub struct Password(String);

impl Password {
    /// Create a new `Password` from a given string, returning an error if
    /// the string does not satisfy the password policy.
    ///
    /// The password policy is as follows:
    ///
    /// - The password must be at least 5 characters long.
    /// - The password must be at most 32 characters long.
    /// - The password must contain at least one uppercase letter.
    /// - The password must contain at least one lowercase letter.
    /// - The password must contain at least one digit.
    ///
    pub fn new(password: &str) -> Result<Self,PasswordError> {
        // validate password
        Self::validator()
            .validate(password)
            .map_err(PasswordError::InvalidPassword)?;
        Ok(Self(Self::hash_password(password)?))
    }

    /// Hash a given password with Argon2, returning an error if any internal
    /// error occurs.
    ///
    /// The password is hashed with the default Argon2 parameters, which should
    /// be suitable for most purposes. The hash is returned as a string.
    ///
    /// # Errors
    ///
    /// - If Argon2 fails to hash the password, an error of type `PasswordError::HashingFailed` is returned.
    fn hash_password(password: &str) -> Result<String, PasswordError> {
        let hasher = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = hasher
            .hash_password(
                password.as_bytes(),
                &salt
            )
            .map(|p_hash| p_hash.to_string())
            .map_err(|_e| PasswordError::HashingFailed)?;
        Ok(hash)
    }

    /// Verify a given password against the stored hash, returning an error if
    /// the password is incorrect or if there is an internal error.
    ///
    /// # Errors
    ///
    /// - If the password is incorrect, an error of type `PasswordError::WrongPassword` is returned.
    /// - If there is an internal error when verifying the password, an error of type `PasswordError::HashParsingFailed` is returned.
    pub fn check_against(&self, password: &str) -> Result<(), PasswordError>{
        if let Ok(target_hash) = PasswordHash::new(&self.0) {
            Argon2::default()
                .verify_password(password.as_bytes(), &target_hash)
                .map_err(|_err| PasswordError::WrongPassword)
        }
        else {
            Err(PasswordError::HashParsingFailed)
        }
        
    }
    /// Returns the stored hash string.
    ///
    /// This is a read-only getter for the stored hash string, which is
    /// the result of hashing the password with Argon2.
    pub fn hash_str(&self) -> &str {
        &self.0
    }

    /// Returns a `Pval` object that defines the password policy.
    ///
    /// The password policy is as follows:
    ///
    /// - The password must be at least 5 characters long.
    /// - The password must be at most 32 characters long.
    /// - The password must contain at least one uppercase letter.
    /// - The password must contain at least one lowercase letter.
    /// - The password must contain at least one digit.
    ///
    /// This policy is used by `Password::new` to validate the password.
    ///
    fn validator() -> Pval {
        Pval::new()
            .min_length(5)
            .max_length(32)
            .require_uppercase(true)
            .require_lowercase(true)
            .require_digit(true)
            .build()
    }
}
