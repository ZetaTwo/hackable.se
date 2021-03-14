/*! Persistent customer storage. */
use auto_impl::auto_impl;

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::RwLock,
};

use crate::{
    db::DbConn,
    domain::{
        error::{err_msg, Error},
        users::{User, UserId},
    },
};

/** A place to persist and fetch users. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait UserStore {
    fn get_user(&self, id: UserId) -> Result<Option<User>, Error>;
    fn set_user(&self, user: User) -> Result<(), Error>;

    //fn create_session(&self, user: User) -> Result<(), Error>; //TODO session
}

/* A test in-memory user store. */
pub(in crate::domain) type InMemoryStore = RwLock<HashMap<UserId, User>>;
pub(in crate::domain) struct DbStore {
    connection: DbConn,
}

impl UserStore for InMemoryStore {
    fn get_user(&self, id: UserId) -> Result<Option<User>, Error> {
        Err(err_msg("Not implemented"))
    }

    fn set_user(&self, user: User) -> Result<(), Error> {
        Err(err_msg("Not implemented"))
    }
}

impl UserStore for DbConn {
    fn get_user(&self, id: UserId) -> Result<Option<User>, Error> {
        Err(err_msg("Not implemented"))
    }

    fn set_user(&self, user: User) -> Result<(), Error> {
        Err(err_msg("Not implemented"))
    }
}

pub(in crate::domain) fn in_memory_store() -> DbStore {
    InMemoryStore {}
}

pub(in crate::domain) fn database_store(conn: DbConn) -> DbStore {
    DbStore { connection: conn }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::users::{model::test_data::UserBuilder, *};
}
