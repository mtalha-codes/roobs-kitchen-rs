use super::type_errors::password_error::PasswordError;
use argon2::{password_hash::rand_core::OsRng, password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use pval::Pval;

#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn new(password: &str) -> Result<Self,PasswordError> {
        // validate password
        Self::validator()
            .validate(password)
            .map_err(PasswordError::InvalidPassword)?;
        Ok(Self(Self::hash_password(password)?))
    }

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
    pub fn hash_str(&self) -> &str {
        &self.0
    }

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
