use crate::db;
use rocket::response::status::NotFound;

// TODO: list all challenges
#[get("/challenges", format = "json")]
pub fn list_challenges(_connection: db::DbConn) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: Get specific challenge
#[get("/challenges/<_challenge_id>", format = "json")]
pub fn get_challenge(
    _connection: db::DbConn,
    _challenge_id: rocket_contrib::uuid::Uuid,
) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: Solve challenge
#[post(
    "/challenges/<_challenge_id>/solves",
    data = "<_solve_request>",
    format = "json"
)]
pub fn solve_challenge(
    _connection: db::DbConn,
    _challenge_id: rocket_contrib::uuid::Uuid,
    _solve_request: String,
) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: Start challenge
#[post(
    "/challenges/<_challenge_id>/instances",
    data = "<_instance_request>",
    format = "json"
)]
pub fn start_challenge(
    _connection: db::DbConn,
    _challenge_id: rocket_contrib::uuid::Uuid,
    _instance_request: String,
) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}
