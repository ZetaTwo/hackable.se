use crate::domain::{
    id::{Id, NextId},
    version::Version,
};

pub mod store;
pub mod tag;

pub use self::tag::*;

pub type ChallengeId = Id<Challenge>;
pub type NextChallengeId = NextId<Challenge>;
pub type ChallengeVersion = Version<Challenge>;

pub struct Challenge {}
