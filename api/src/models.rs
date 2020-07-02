use serde::{Deserialize, Serialize};

pub mod email;
pub mod id;
pub mod password;
pub mod user;
pub mod username;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResult {
    pub code: u32,
    pub message: String,
}
