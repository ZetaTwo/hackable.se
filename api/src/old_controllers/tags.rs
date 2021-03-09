use crate::db;
use rocket::response::status::NotFound;

// TODO: list all tags
#[get("/tags", format = "json")]
pub fn list_tags(_connection: db::DbConn) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}

// TODO: fetch tag, include associated challenges
#[get("/tags/<_tag_id>", format = "json")]
pub fn get_tag(_connection: db::DbConn, _tag_id: rocket_contrib::uuid::Uuid) -> NotFound<String> {
    NotFound(format!("Not implemented"))
}
