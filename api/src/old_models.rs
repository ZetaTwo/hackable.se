use serde::{Deserialize, Serialize};

pub mod challenge;
pub mod email;
pub mod id;
pub mod password;
pub mod password_hash;
pub mod session;
pub mod solve;
pub mod tag;
pub mod user;
pub mod username;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResult {
    pub code: u32,
    pub message: String,
}
