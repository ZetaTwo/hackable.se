use crate::domain::{
    version::Version,
};

pub type SessionId = Id<SessionData>;
pub type NextSessionId = NextId<SessionData>;
pub type SessionVersion = Version<SessionData>;

pub struct Session {

};
