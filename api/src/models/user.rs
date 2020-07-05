use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use super::email::{Email, EmailValidationError};
use super::id::UUID;
use super::password::{Password, PasswordValidationError};
use super::password_hash::{PasswordHash, PasswordHashError, PasswordHashingConfig};
use super::username::{Username, UsernameValidationError};
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: UUID,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub email: Email,
    pub email_validated: bool,
    pub created: NaiveDateTime,
}

impl From<User> for UserPublicInfo {
    fn from(user: User) -> UserPublicInfo {
        UserPublicInfo {
            id: user.id,
            username: user.username,
        }
    }
}

impl From<User> for UserPrivateInfo {
    fn from(user: User) -> UserPrivateInfo {
        UserPrivateInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            email_validated: user.email_validated,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl UserRegistrationRequest {
    pub fn validate(
        self,
        password_hashing_config: &PasswordHashingConfig,
    ) -> Result<UserRegistration, RegistrationValidationError> {
        let username = Username::try_from(self.username);
        let password_hash = Password::try_from(self.password)?.hash(password_hashing_config);
        let email = Email::try_from(self.email);

        // TODO: return all errors instead of just first error
        Ok(UserRegistration {
            username: username?,
            password_hash: password_hash?,
            email: email?,
        })
    }
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegistration {
    pub username: Username,
    pub password_hash: PasswordHash,
    pub email: Email,
}

#[derive(Debug)]
pub enum RegistrationValidationError {
    Email(EmailValidationError),
    Username(UsernameValidationError),
    PasswordValidation(PasswordValidationError),
    PasswordHash(PasswordHashError),
}

impl From<EmailValidationError> for RegistrationValidationError {
    fn from(err: EmailValidationError) -> RegistrationValidationError {
        RegistrationValidationError::Email(err)
    }
}

impl From<UsernameValidationError> for RegistrationValidationError {
    fn from(err: UsernameValidationError) -> RegistrationValidationError {
        RegistrationValidationError::Username(err)
    }
}

impl From<PasswordValidationError> for RegistrationValidationError {
    fn from(err: PasswordValidationError) -> RegistrationValidationError {
        RegistrationValidationError::PasswordValidation(err)
    }
}

impl From<PasswordHashError> for RegistrationValidationError {
    fn from(err: PasswordHashError) -> RegistrationValidationError {
        RegistrationValidationError::PasswordHash(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdate {
    #[serde(flatten)]
    pub password: Password,
    #[serde(flatten)]
    pub email: Email,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublicInfo {
    pub id: UUID,
    pub username: Username,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserPrivateInfo {
    pub id: UUID,
    pub username: Username,
    pub email: Email,
    pub email_validated: bool,
}
