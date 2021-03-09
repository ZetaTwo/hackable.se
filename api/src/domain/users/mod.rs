/*! Domain module for user. */

pub mod commands;
pub mod model;
pub mod queries;
pub mod resolver;

pub use self::{
    commands::*,
    model::*,
    queries::*,
    resolver::*,
};

pub(self) use self::model::store::UserStore;
