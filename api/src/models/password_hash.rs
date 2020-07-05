use argonautica::Hasher;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::ops::Deref;

use super::password::Password;

pub struct PasswordHashingConfig {
    secret_key: String,
}

impl PasswordHashingConfig {
    pub fn new(secret_key: String) -> PasswordHashingConfig {
        PasswordHashingConfig {
            secret_key: secret_key,
        }
    }
}

// TODO: Add display
#[derive(Debug)]
pub enum PasswordHashError {
    Hash(argonautica::Error),
}

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub struct PasswordHash {
    password_hash: String,
}

impl PasswordHash {
    pub fn hash_password(
        password: &Password,
        hash_config: &PasswordHashingConfig,
    ) -> Result<Self, PasswordHashError> {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(&**password)
            .with_secret_key(&hash_config.secret_key)
            .hash();

        match hash {
            Ok(hash) => Ok(PasswordHash {
                password_hash: hash,
            }),
            // TODO: Granular error handling
            Err(err) => Err(PasswordHashError::Hash(err)),
        }
    }
}

impl Deref for PasswordHash {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.password_hash
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
