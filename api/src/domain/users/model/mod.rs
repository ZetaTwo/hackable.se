/*! Contains the `User` entity. */

use std::convert::{
    //TryFrom,
    TryInto,
};

use crate::domain::{
    entity::Entity,
    error::Error,
    id::{
        Id,
        IdProvider,
        NextId,
    },
    users::{
        Email,
        Username,
    },
    version::Version,
    Resolver,
};

pub mod store;

pub type UserId = Id<UserData>;
pub type NextUserId = NextId<UserData>;
pub type UserVersion = Version<UserData>;

#[cfg(test)]
pub mod test_data;

/** Data for a user. */
#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: UserId,
    pub version: UserVersion,
    pub username: Username,
    pub email: Email,
    pub email_validated: bool,
    _private: (),
}

/** A user. */
pub struct User {
    data: UserData,
}

impl User {
    pub(self) fn from_data(data: UserData) -> Self {
        User { data: data }
    }

    pub fn to_data(&self) -> &UserData {
        &self.data
    }

    pub fn into_data(self) -> UserData {
        self.data
    }

    pub fn new<TId, TUsername, TEmail, TEmailValidated>(
        id_provider: TId,
        username: TUsername,
        email: TEmail,
        email_validated: TEmailValidated,
    ) -> Result<Self, Error>
    where
        TId: IdProvider<UserData>,
        TUsername: TryInto<Username, Error = Error>,
        TEmail: TryInto<Email, Error = Error>,
        TEmailValidated: TryInto<bool, Error = Error>,
    {
        let id = id_provider.id()?;

        Ok(User::from_data(UserData {
            id: id,
            version: UserVersion::default(),
            username: username.try_into()?.0,
            email: email.try_into()?.0,
            email_validated: email_validated.try_into()?,
            _private: (),
        }))
    }
}

impl Entity for User {
    type Id = UserId;
    type Version = UserVersion;
    type Data = UserData;
    type Error = Error;
}

impl Resolver {
    pub fn user_id_provider(&self) -> impl IdProvider<UserData> {
        NextId::<UserData>::new()
    }
}
