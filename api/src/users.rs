use crate::db;
use crate::diesel::prelude::*;
use crate::models::*;

use crate::models::id::UUID;
use rocket::response::status::{Conflict, NotFound};
use rocket_contrib::json::Json;

use crate::users::email::Email;
use crate::users::username::Username;

#[get("/users/<user_id>")]
pub fn get_user(
    connection: db::DbConn,
    user_id: rocket_contrib::uuid::Uuid,
) -> Result<Json<UserPublicInfo>, NotFound<String>> {
    use crate::schema::users::dsl::*;

    match users
        .find(UUID::from(user_id))
        .get_result::<User>(&*connection)
    {
        Err(_) => Err(NotFound(format!("User {} not found", user_id.to_string()))),
        Ok(user) => Ok(Json(UserPublicInfo {
            id: user.id,
            username: user.username,
        })),
    }
}

#[post("/users", data = "<registration>", format = "json")]
pub fn create_user(
    connection: db::DbConn,
    registration: Json<UserRegistration>,
) -> Result<Json<UserPrivateInfo>, Conflict<String>> {
    use crate::schema::users::dsl::*;

    let creation_data = UserCreationData::from(registration.into_inner());

    let insert_result = diesel::insert_into(users)
        .values(creation_data)
        .execute(&*connection);

    match insert_result {
        Err(_) => Err(Conflict(Some(format!("User already exists.")))),
        //TODO: Actually return the created user
        Ok(v) => Ok(Json(UserPrivateInfo {
            id: UUID::parse_str("123e4567-e89b-12d3-a456-426655440000").unwrap(),
            username: Username::from("USERNAME".to_string()),
            email: Email::from("EMAIL".to_string()),
            email_validated: false,
        })),
    }
}
