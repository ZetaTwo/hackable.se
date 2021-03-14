/*! Contains the root `Resolver` type. */

use crate::{
    db::DbConn,
    domain::{challenges::resolver::ChallengesResolver, users::resolver::UsersResolver},
};

/**
Resolver for the domain.
The goal of the resolver is to let consumers construct components without having to know what their dependencies are.
The `Resolver` type wraps resolvers from other modules.
Private implementation details live on the wrapped resolvers.
Commands and queries are resolved from this `Resolver`.
*/
pub struct Resolver {
    user_resolver: UsersResolver,
    challenge_resolver: ChallengesResolver,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            user_resolver: UsersResolver::default(),
            challenge_resolver: ChallengesResolver::default(),
        }
    }
}

impl Resolver {
    pub fn database_resolver(conn: DbConn) -> Self {
        Resolver {
            user_resolver: UsersResolver::database_resolver(conn),
            challenge_resolver: ChallengesResolver::database_resolver(conn),
        }
    }

    pub(in crate::domain) fn users(&self) -> &UsersResolver {
        &self.user_resolver
    }

    pub(in crate::domain) fn challenges(&self) -> &ChallengesResolver {
        &self.challenge_resolver
    }
}
