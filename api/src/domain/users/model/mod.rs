/*! Contains the `User` entity. */

use std::convert::TryInto;

use crate::domain::{
    entity::Entity,
    error::Error,
    id::{Id, IdProvider, NextId},
    users::{Email, Username},
    version::Version,
    Resolver,
};

pub mod store;

pub use email::*;
pub use session::*;
pub use username::*;

pub type UserId = Id<User>;
pub type NextUserId = NextId<User>;
pub type UserVersion = Version<User>;

#[cfg(test)]
pub mod test_data;

/** A user. */
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub version: UserVersion,
    pub username: Username,
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
            username: username.try_into()?.0,
            email: email.try_into()?.0,
            email_validated: email_validated.try_into()?,
            _private: (),
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
