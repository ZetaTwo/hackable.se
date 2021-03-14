use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt;

use super::password_hash::{PasswordHash, PasswordHashError, PasswordHashingConfig};

const PASSWORD_MIN_LENGTH: usize = 8;

#[derive(Debug, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn hash(
        &self,
        hash_config: &PasswordHashingConfig,
    ) -> Result<PasswordHash, PasswordHashError> {
        PasswordHash::hash_password(self, hash_config)
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordValidationError;

    fn try_from(password: String) -> Result<Self, Self::Error> {
        if password.len() < PASSWORD_MIN_LENGTH {
            Err(PasswordValidationError::Format)
        } else {
            Ok(Password(password))
        }
    }
}

impl<'a> TryFrom<&'a str> for Password {
    type Error = Error;

    fn try_from(password: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(password.to_owned())
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// TODO: Add display
#[derive(Debug)]
pub enum PasswordValidationError {
    Format,
}
