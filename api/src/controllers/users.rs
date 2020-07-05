use crate::db;
use crate::diesel::prelude::*;

use rocket::response::status::NotFound;
use rocket::response::Responder;

use rocket_contrib::json::Json;

use crate::models::id::UUID;
use crate::models::user::UserPublicInfo;

use crate::models::user::User;
use crate::models::user::UserPrivateInfo;
use crate::models::user::UserRegistrationRequest;
use crate::models::user::UserUpdate;

use crate::ApiResult;

#[get("/users/<user_id>")]
pub fn get_user(
    connection: db::DbConn,
    user_id: rocket_contrib::uuid::Uuid,
) -> Result<Json<UserPublicInfo>, NotFound<String>> {
    use crate::schema::users::dsl::*;

    // TODO: Select and solves/score as well

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

#[derive(Responder, Debug)]
pub enum RegistrationError {
    #[response(status = 400, content_type = "json")]
    Validation(String), //(Vec<ValidationError>),
    #[response(status = 409, content_type = "json")]
    Conflict(String), //(Vec<ValidationError>),
}

#[post("/users", data = "<registration_request>", format = "json")]
pub fn create_user(
    connection: db::DbConn,
    registration_request: Json<UserRegistrationRequest>,
) -> Result<Json<UserPrivateInfo>, RegistrationError> {
    use crate::schema::users::dsl::*;

    // TODO: Validate email basic format

    // TODO: Better error handling
    let registration = registration_request
        .into_inner()
        .validate()
        .map_err(|err| RegistrationError::Validation(format!("Invalid request")))?;

    // Create the new user
    let insert_result = diesel::insert_into(users)
        .values(&registration)
        .execute(&*connection);

    // Select back the newly created user
    let select_result = insert_result.and_then(|_| {
        users
            .select((id, username, email, email_validated))
            .filter(username.eq(&registration.username))
            .first::<UserPrivateInfo>(&*connection)
    });

    match select_result {
        // TODO: Handle different errors granularly
        Err(_) => Err(RegistrationError::Conflict(format!("User already exists"))),
        Ok(user) => Ok(Json(user)),
    }
}

#[patch("/users/<user_id>", data = "<update>", format = "json")]
pub fn update_user(
    connection: db::DbConn,
    update: Json<UserUpdate>,
    user_id: rocket_contrib::uuid::Uuid,
) -> Result<Json<UserPrivateInfo>, NotFound<String>> {
    use crate::schema::users::dsl::*;

    // TODO: Check session user = target user

    let select_user = users
        .find(UUID::from(user_id))
        .get_result::<User>(&*connection);

    // TODO: Actually update the user data

    let update_user = select_user.and_then(|user| Ok(UserPrivateInfo::from(user)));

    match update_user {
        // TODO: Handle different errors granularly
        Err(_) => Err(NotFound(format!("User {} not found", user_id.to_string()))),
        Ok(user) => Ok(Json(user)),
    }
}

#[delete("/users/<user_id>")]
pub fn delete_user(
    connection: db::DbConn,
    user_id: rocket_contrib::uuid::Uuid,
) -> Result<Json<ApiResult>, NotFound<String>> {
    use crate::schema::users::dsl::*;

    // TODO: Don't implement this at all?

    match users
        .find(UUID::from(user_id))
        .get_result::<User>(&*connection)
    {
        Err(_) => Err(NotFound(format!("User {} not found", user_id.to_string()))),
        Ok(user) => Ok(Json(ApiResult {
            code: 200,
            message: "User not actually deleted".to_string(),
        })),
    }
}
