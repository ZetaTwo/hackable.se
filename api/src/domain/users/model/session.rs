use crate::domain::{
    id::{Id, NextId},
    version::Version,
};

pub type SessionId = Id<Session>;
pub type NextSessionId = NextId<Session>;
pub type SessionVersion = Version<Session>;

pub struct Session {
    id: SessionId,
}
