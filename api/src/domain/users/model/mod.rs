/*! Contains the `User` entity. */

use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

pub mod email;
pub mod password;
pub mod password_hash;
pub mod session;
pub mod username;

pub use email::*;
pub use password::*;
pub use password_hash::*;
pub use session::*;
pub use username::*;

use crate::domain::{
    entity::Entity,
    error::Error,
    id::{Id, IdProvider, NextId},
    version::Version,
    Resolver,
};

pub mod store;

pub type UserId = Id<User>;
pub type NextUserId = NextId<User>;
pub type UserVersion = Version<User>;

#[cfg(test)]
pub mod test_data;

/** A user. */
pub struct User {
    pub id: UserId,
    pub version: UserVersion,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub email: Email,
    pub email_validated: bool,
}

impl User {
    pub fn new<TId, TUsername, TEmail, TEmailValidated>(
        id_provider: TId,
        username: TUsername,
        email: TEmail,
        email_validated: TEmailValidated,
    ) -> Result<Self, Error>
    where
        TId: IdProvider<User>,
        TUsername: TryInto<Username, Error = Error>,
        TEmail: TryInto<Email, Error = Error>,
        TEmailValidated: TryInto<bool, Error = Error>,
    {
        let id = id_provider.id()?;

        Ok(User {
            id: id,
            version: UserVersion::default(),
            username: username.try_into()?,
            email: email.try_into()?,
            email_validated: email_validated.try_into()?,
        })
    }
}

impl Entity for User {
    type Id = UserId;
    type Version = UserVersion;
    //type Data = UserData;
    type Error = Error;
}

impl Resolver {
    pub fn user_id_provider(&self) -> impl IdProvider<User> {
        NextId::<User>::new()
    }
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

#[derive(Debug, Insertable)]
#[table_name = "users"]
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

#[derive(Deserialize, Debug)]
pub struct UserUpdate {
    pub password: Password,
    pub email: Email,
}

#[derive(Serialize, Debug)]
pub struct UserPublicInfo {
    pub id: UserId,
    pub username: Username,
}

#[derive(Queryable, Serialize, Debug)]
pub struct UserPrivateInfo {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub email_validated: bool,
}
