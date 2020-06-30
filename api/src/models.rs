pub mod id;
use id::UUID;

pub mod password;
use password::Password;

pub mod email;
use email::Email;

pub mod username;
use username::Username;

use crate::schema::users;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResult {
    pub code: u32,
    pub message: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: UUID,
    pub username: Username,
    pub password: Password,
    pub email: Email,
    pub email_validated: bool,
}

impl From<User> for UserPublicInfo {
    fn from(user: User) -> UserPublicInfo {
        UserPublicInfo {
            id: user.id,
            username: user.username,
        }
    }
}

impl From<User> for UserPrivateInfo {
    fn from(user: User) -> UserPrivateInfo {
        UserPrivateInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            email_validated: user.email_validated,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegistration {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationData {
    pub username: Username,
    pub password: Password,
    pub email: Email,
}

impl From<UserRegistration> for UserCreationData {
    fn from(user_registration: UserRegistration) -> UserCreationData {
        UserCreationData {
            username: Username::from(user_registration.username),
            password: Password::from(user_registration.password),
            email: Email::from(user_registration.email),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublicInfo {
    pub id: UUID,
    pub username: Username,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPrivateInfo {
    pub id: UUID,
    pub username: Username,
    pub email: Email,
    pub email_validated: bool,
}
