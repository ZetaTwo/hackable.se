use crate::db;
use crate::models::*;
use crate::diesel::prelude::*;

use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use crate::models::id::UUID;


#[get("/users/<user_id>")]
pub fn get_user(connection: db::DbConn, user_id: rocket_contrib::uuid::Uuid) -> Result<Json<UserPublicInfo>, NotFound<String>> {
    use crate::schema::users::dsl::*;

    match users.find(UUID::from(user_id)).get_result::<User>(&*connection) {
        Err(_) => Err(NotFound(format!("User {} not found", user_id.to_string()))),
        Ok(user) => Ok(Json(UserPublicInfo { id: user.id, username: user.username })),
    }
}


#[post("/users", data = "<registration>", format = "json")]
pub fn create_user(connection: db::DbConn, registration: Json<UserRegistration>) -> Json<UserPrivateInfo> {
    println!("Username: {}", registration.username);
    println!("Password: {}", registration.password);
    println!("Email: {}", registration.email);
    
    Json(UserPrivateInfo { id: UUID::parse_str("123e4567-e89b-12d3-a456-426655440000").unwrap(), username: registration.username.to_string(), email: registration.email.to_string(), email_validated: false })
}