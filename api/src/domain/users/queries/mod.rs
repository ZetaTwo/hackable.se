/*! Queries for fetching User state. */

pub mod get_user;
pub use self::get_user::{GetUser, GetUserQuery};

pub mod get_user_with_solves;
pub use self::get_user_with_solves::{GetUserWithSolves, GetUserWithSolvesQuery, UserWithSolves};
