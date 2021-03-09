use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::ops::Deref;

const EMAIL_MIN_LENGTH: usize = 5;

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub struct Email(String);

impl<DB> FromSql<Text, DB> for Email
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(email_bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(Email {
            email: String::from_sql(email_bytes)?,
        })
    }
}

impl<DB> ToSql<Text, DB> for Email
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.email.to_sql(out)
    }
}

// Add display
#[derive(Debug)]
pub enum EmailValidationError {
    FormatError,
}

impl TryFrom<String> for Email {
    type Error = EmailValidationError;

    fn try_from(email: String) -> Result<Self, Self::Error> {
        // TODO: Add email format validation

        if email.len() < EMAIL_MIN_LENGTH {
            Err(EmailValidationError::FormatError)
        } else {
            Ok(Email { email: email })
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
