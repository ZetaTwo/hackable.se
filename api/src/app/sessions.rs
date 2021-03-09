/*! `/sessions` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{
        err_msg,
        Error,
    },
    domain::{
        id::IdProvider,
        users::*,
        Resolver,
    },
};

/** `GET /sessions/<id>` */
#[get("/<id>")]
pub fn get(id: SessionId, resolver: State<Resolver>) -> Result<Json<Session>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `PUT /sessions` */
#[put("/")]
pub fn refresh(id: SessionId, resolver: State<Resolver>) -> Result<Json<Session>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `POST /sessions` */
#[post("/", format = "application/json")]
pub fn login(resolver: State<Resolver>) -> Result<Json<Session>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `DELETE /sessions/<id>` */
#[delete("/")]
pub fn delete(id: UserId, resolver: State<Resolver>) -> Result<Json<SessionId>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
