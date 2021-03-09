/*! Contains the `UsersResolver` type. */

use std::sync::Arc;

use crate::domain::users::model::store as user_store;

/**
Resolver for users.
The `UsersResolver` type wraps private implementation details and exposes them as traits within the `users` module.
*/
pub struct UsersResolver {
    user_store: Arc<user_store::InMemoryStore>,
}

impl Default for UsersResolver {
    fn default() -> Self {
        UsersResolver {
            user_store: Arc::new(user_store::in_memory_store()),
        }
    }
}

impl UsersResolver {
    pub(in crate::domain::users) fn user_store(
        &self,
    ) -> impl user_store::UserStore {
        self.user_store.clone()
    }
}
