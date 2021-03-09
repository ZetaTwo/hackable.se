/*! `/tags` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{
        err_msg,
        Error,
    },
    domain::{
        id::IdProvider,
        challenges::*,
        Resolver,
    },
};

/** `GET /tags/<id>` */
#[get("/<id>")]
pub fn get(id: TagId, resolver: State<Resolver>) -> Result<Json<Tag>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `GET /tags` */
#[get("/")]
pub fn list(resolver: State<Resolver>) -> Result<Json<Tag>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
