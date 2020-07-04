use super::email::{Email, EmailValidationError};
use super::id::UUID;
use super::password::{Password, PasswordHash, PasswordValidationError, PasswordHashError};
use super::username::{Username, UsernameValidationError};
use std::convert::TryFrom;

use crate::schema::users;

use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn validate(self) -> Result<UserRegistration, RegistrationValidationEror> {
        UserRegistration::try_from(self)
    }
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegistration {
    #[serde(flatten)]
    pub username: Username,
    #[serde(flatten)]
    pub password_hash: PasswordHash,
    #[serde(flatten)]
    pub email: Email,
}

#[derive(Debug)]
pub enum RegistrationValidationEror {
    Email(EmailValidationError),
    Username(UsernameValidationError),
    PasswordValidation(PasswordValidationError),
    PasswordHash(PasswordHashError),
}

impl From<EmailValidationError> for RegistrationValidationEror {
    fn from(err: EmailValidationError) -> RegistrationValidationEror {
        RegistrationValidationEror::Email(err)
    }
}

impl From<UsernameValidationError> for RegistrationValidationEror {
    fn from(err: UsernameValidationError) -> RegistrationValidationEror {
        RegistrationValidationEror::Username(err)
    }
}

impl From<PasswordValidationError> for RegistrationValidationEror {
    fn from(err: PasswordValidationError) -> RegistrationValidationEror {
        RegistrationValidationEror::PasswordValidation(err)
    }
}

impl From<PasswordHashError> for RegistrationValidationEror {
    fn from(err: PasswordHashError) -> RegistrationValidationEror {
        RegistrationValidationEror::PasswordHash(err)
    }
}


impl TryFrom<UserRegistrationRequest> for UserRegistration {
    type Error = RegistrationValidationEror;

    fn try_from(registration_request: UserRegistrationRequest) -> Result<Self, Self::Error> {
        let username = Username::try_from(registration_request.username);
        let password_hash = Password::try_from(registration_request.password)?.hash();
        let email = Email::try_from(registration_request.email);

        // TODO: return all errors instead of just first error
        Ok(UserRegistration {
            username: username?,
            password_hash: password_hash?,
            email: email?
         })
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
