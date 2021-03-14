/*! Contains the `ChallengesResolver` type. */

use std::sync::Arc;

use crate::{db::DbConn, domain::challenges::model::store as challenge_store};

/**
Resolver for challenges.
The `ChallengesResolver` type wraps private implementation details and exposes them as traits within the `challenges` module.
*/
pub struct ChallengesResolver {
    challenge_store: Arc<challenge_store::InMemoryStore>,
}

impl Default for ChallengesResolver {
    fn default() -> Self {
        ChallengesResolver {
            challenge_store: Arc::new(challenge_store::in_memory_store()),
        }
    }
}

impl ChallengesResolver {
    pub fn database_resolver(conn: DbConn) -> Self {
        ChallengesResolver {
            challenge_store: ChallengesResolver::database_resolver(conn),
        }
    }

    pub(in crate::domain::challenges) fn challenge_store(&self) -> impl challenge_store::UserStore {
        self.challenge_store.clone()
    }
}
