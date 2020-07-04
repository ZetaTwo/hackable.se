use std::fmt;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::ops::Deref;
use std::convert::TryFrom;

use argonautica::Hasher;

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

/*
impl From<String> for Password {
    fn from(password: String) -> Self {
        Password { password: password }
    }
}

impl From<&str> for Password {
    fn from(password: &str) -> Self {
        Password {
            password: password.to_string(),
        }
    }
}
*/

impl TryFrom<String> for Password {
    type Error = PasswordValidationError;

    fn try_from(password: String) -> Result<Self, Self::Error> {
        // TODO: Actually add validation
        Ok(Password { password: password })
    }
}


impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub enum PasswordValidationError {
    Format,
    Hash(PasswordHashError)
}

#[derive(Debug)]
pub enum PasswordHashError {
    Hash(argonautica::Error),
}

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub struct PasswordHash {
    password_hash: String,
}

impl TryFrom<&Password> for PasswordHash {
    type Error = PasswordHashError;

    fn try_from(password: &Password) -> Result<Self, Self::Error> {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(&**password)
            .with_secret_key("TODO: Change to env")
            .hash();

        match hash {
            Ok(hash) => Ok(PasswordHash { password_hash: hash }),
            // TODO: Granular error handling
            Err(err) => Err(PasswordHashError::Hash(err))
        }
    }
}

impl<DB> FromSql<Text, DB> for PasswordHash
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(password_hash_bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(PasswordHash {
            password_hash: String::from_sql(password_hash_bytes)?,
        })
    }
}

impl<DB> ToSql<Text, DB> for PasswordHash
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.password_hash.to_sql(out)
    }
}
