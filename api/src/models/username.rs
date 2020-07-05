use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::ops::Deref;

const USERNAME_MIN_LENGTH: usize = 5;

#[derive(Debug, AsExpression, Serialize, Deserialize, FromSqlRow)]
#[sql_type = "Text"]
pub struct Username {
    username: String,
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.username
    }
}

impl<DB> FromSql<Text, DB> for Username
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(username_bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(Username {
            username: String::from_sql(username_bytes)?,
        })
    }
}

impl<DB> ToSql<Text, DB> for Username
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.username.to_sql(out)
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// TODO: Add display
#[derive(Debug)]
pub enum UsernameValidationError {
    FormatError,
}

impl TryFrom<String> for Username {
    type Error = UsernameValidationError;

    fn try_from(username: String) -> Result<Self, Self::Error> {
        // TODO: Add username character validation

        if username.len() < USERNAME_MIN_LENGTH {
            Err(UsernameValidationError::FormatError)
        } else {
            Ok(Username { username: username })
        }
    }
}
