/*! Contains the `GetUserQuery` type. */

use auto_impl::auto_impl;

use crate::domain::{
    error::Error,
    users::{User, UserId, UserStore},
    Resolver,
};

pub type Result = ::std::result::Result<Option<User>, Error>;

/** Input for a `GetUserQuery`. */
#[derive(Deserialize)]
pub struct GetUser {
    pub id: UserId,
}

/** Get a user entity. */
#[auto_impl(Fn)]
pub trait GetUserQuery {
    fn get_user(&self, query: GetUser) -> Result;
}

/** Default implementation for a `GetUserQuery`. */
pub(in crate::domain) fn get_user_query(store: impl UserStore) -> impl GetUserQuery {
    move |query: GetUser| {
        let user = store.get_user(query.id)?;

        Ok(user)
    }
}

impl Resolver {
    pub fn get_user_query(&self) -> impl GetUserQuery {
        let store = self.users().user_store();

        get_user_query(store)
    }
}
