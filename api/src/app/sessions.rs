/*! `/sessions` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{err_msg, Error},
    domain::{users::*, Resolver},
};

// TODO: Fetch session
// TODO: Check that current session = target session
/** `GET /sessions/<id>` */
#[get("/<session_id>")]
pub fn get(session_id: SessionId, resolver: State<Resolver>) -> Result<Json<Session>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

// TODO: update session, aka. renew
/** `PUT /sessions` */
#[put(
    "/<session_id>",
    data = "<refresh_request>",
    format = "application/json"
)]
pub fn refresh(
    session_id: SessionId,
    refresh_request: String,
    resolver: State<Resolver>,
) -> Result<Json<Session>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

// TODO: create session aka. login
/** `POST /sessions` */
#[post("/", data = "<login_request>", format = "application/json")]
pub fn login(resolver: State<Resolver>, login_request: String) -> Result<Json<Session>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

// TODO: delete session, aka. logout
/** `DELETE /sessions/<id>` */
#[delete("/<session_id>")]
pub fn delete(session_id: SessionId, resolver: State<Resolver>) -> Result<Json<SessionId>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
