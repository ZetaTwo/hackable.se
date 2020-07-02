use crate::db;
use crate::diesel::prelude::*;

use rocket::response::status::{Conflict, NotFound};
use rocket_contrib::json::Json;

use crate::models::id::UUID;
use crate::models::user::UserPublicInfo;

use crate::models::user::User;
use crate::models::user::UserPrivateInfo;
use crate::models::user::UserRegistration;

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

    // Create the new user
    let insert_result = diesel::insert_into(users)
        .values(&*registration)
        .execute(&*connection);

    // Select back the newly created user
    let select_result = insert_result.and_then(|_| {
        users
            .select((id, username, email, email_validated))
            .filter(username.eq(&registration.username))
            .first::<UserPrivateInfo>(&*connection)
    });

    match select_result {
        Err(_) => Err(Conflict(Some(format!("User already exists.")))),
        Ok(user) => Ok(Json(user)),
    }
}
