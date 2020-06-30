use std::fmt;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::ops::Deref;

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub struct Password {
    password: String,
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.password
    }
}

impl<DB> FromSql<Text, DB> for Password
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(password_bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(Password {
            password: String::from_sql(password_bytes)?,
        })
    }
}

impl<DB> ToSql<Text, DB> for Password
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.password.to_sql(out)
    }
}

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

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PASSWORD HASH: {}", self)
    }
}
