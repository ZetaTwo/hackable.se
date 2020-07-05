use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use super::password_hash::{PasswordHash, PasswordHashError};

const PASSWORD_MIN_LENGTH: usize = 8;

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn hash(&self) -> Result<PasswordHash, PasswordHashError> {
        PasswordHash::try_from(self)
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.password
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordValidationError;

    fn try_from(password: String) -> Result<Self, Self::Error> {
        if password.len() < PASSWORD_MIN_LENGTH {
            Err(PasswordValidationError::Format)
        } else {
            Ok(Password { password: password })
        }
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
