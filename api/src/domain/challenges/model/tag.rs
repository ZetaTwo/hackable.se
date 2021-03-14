use crate::domain::{
    id::{Id, NextId},
    version::Version,
};

pub type TagId = Id<Tag>;
pub type NextTagId = NextId<Tag>;
pub type TagVersion = Version<Tag>;

pub struct Tag {}
