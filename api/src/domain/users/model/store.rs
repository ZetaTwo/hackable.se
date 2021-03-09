/*! Persistent customer storage. */
use auto_impl::auto_impl;

use std::{
    collections::{
        hash_map::Entry,
        HashMap,
    },
    sync::RwLock,
};

use crate::domain::{
    users::{
        User,
        UserData,
        UserId,
    },
    error::{
        err_msg,
        Error,
    },
};

/** A place to persist and fetch users. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait UserStore {
    fn get_user(&self, id: UserId) -> Result<Option<User>, Error>;
    fn set_user(&self, customer: User) -> Result<(), Error>;
}

/* A test in-memory customer store. */
pub(in crate::domain) type InMemoryStore = RwLock<HashMap<UserId, UserData>>;

impl UserStore for InMemoryStore {
    fn get_user(&self, id: UserId) -> Result<Option<User>, Error> {
        let users = self.read().map_err(|_| err_msg("not good!"))?;

        if let Some(data) = users.get(&id) {
            Ok(Some(User::from_data(data.clone())))
        } else {
            Ok(None)
        }
    }

    fn set_user(&self, customer: User) -> Result<(), Error> {
        let mut data = customer.into_data();
        let id = data.id;

        let mut users = self.write().map_err(|_| err_msg("not good!"))?;

        match users.entry(id) {
            Entry::Vacant(entry) => {
                data.version.next();
                entry.insert(data);
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.version != data.version {
                    Err(err_msg("optimistic concurrency fail"))?
                }

                data.version.next();
                *entry = data;
            }
        }

        Ok(())
    }
}

pub(in crate::domain) fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::users::{
        model::test_data::UserBuilder,
        *,
    };

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store();

        let id = UserId::new();

        // Create a customer in the store
        store
            .set_user(UserBuilder::new().id(id).build())
            .unwrap();

        // Get the customer from the store
        let found = store.get_user(id).unwrap().unwrap();
        assert_eq!(id, found.data.id);
    }

    #[test]
    fn add_user_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let id = UserId::new();

        // Create a customer in the store
        store
            .set_user(UserBuilder::new().id(id).build())
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(store
            .set_user(UserBuilder::new().id(id).build())
            .is_err());
    }
}
