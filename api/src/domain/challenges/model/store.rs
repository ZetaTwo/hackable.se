/*! Persistent challenge storage. */
use auto_impl::auto_impl;

use std::{collections::HashMap, sync::RwLock};

use crate::{
    db::DbConn,
    domain::{
        challenges::{Challenge, ChallengeId},
        error::{err_msg, Error},
    },
};

/** A place to persist and fetch challenge. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait ChallengeStore {
    fn get_user(&self, id: ChallengeId) -> Result<Option<Challenge>, Error>;
    fn set_user(&self, user: Challenge) -> Result<(), Error>;

    //fn create_session(&self, user: Challenge) -> Result<(), Error>; //TODO session
}

pub(in crate::domain) type InMemoryStore = RwLock<HashMap<ChallengeId, Challenge>>;
pub(in crate::domain) struct DbStore {
    connection: DbConn,
}

impl ChallengeStore for InMemoryStore {
    fn get_user(&self, id: ChallengeId) -> Result<Option<Challenge>, Error> {
        Err(err_msg("Not implemented"))
    }

    fn set_user(&self, user: Challenge) -> Result<(), Error> {
        Err(err_msg("Not implemented"))
    }
}

impl ChallengeStore for DbConn {
    fn get_user(&self, id: ChallengeId) -> Result<Option<Challenge>, Error> {
        Err(err_msg("Not implemented"))
    }

    fn set_user(&self, user: Challenge) -> Result<(), Error> {
        Err(err_msg("Not implemented"))
    }
}

pub(in crate::domain) fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
}

pub(in crate::domain) fn database_store(conn: DbConn) -> DbStore {
    DbStore { connection: conn }
}
