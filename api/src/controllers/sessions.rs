use crate::db;
use rocket::response::status::NotFound;

// TODO: Fetch session
// TODO: Check that current session = target session
#[get("/sessions/<_session_id>", format = "json")]
pub fn get_session(
    _connection: db::DbConn,
    _session_id: rocket_contrib::uuid::Uuid,
) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: create session aka. login
#[post("/sessions", data = "<_session_creation>", format = "json")]
pub fn create_session(_connection: db::DbConn, _session_creation: String) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: update session, aka. renew
#[patch("/sessions/<_session_id>", data = "<_update>", format = "json")]
pub fn update_session(
    _connection: db::DbConn,
    _update: String,
    _session_id: rocket_contrib::uuid::Uuid,
) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: delete session, aka. logout
#[delete("/sessions/<_session_id>")]
pub fn delete_session(
    _connection: db::DbConn,
    _session_id: rocket_contrib::uuid::Uuid,
) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}
