pub mod id;
use id::UUID;

use serde::{Serialize, Deserialize};

#[derive(Queryable)]
pub struct User {
    pub id: UUID,
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_validated: bool,
}

impl Into<UserPublicInfo> for User {
    fn into(self) -> UserPublicInfo {
        UserPublicInfo { id: self.id, username: self.username }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegistration {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublicInfo {
    pub id: UUID,
    pub username: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserPrivateInfo {
    pub id: UUID,
    pub username: String,
    pub email: String,
    pub email_validated: bool,
}
