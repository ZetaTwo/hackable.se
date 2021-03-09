/*! `/challenges` */

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

#[derive(Serialize)]
pub struct Get {
    pub id: ProductId,
    pub title: String,
    pub price: f32,
}

/** `GET /users/<id>` */
#[get("/<id>")]
pub fn get(id: ChallengeId, resolver: State<Resolver>) -> Result<Json<Challenge>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `GET /users` */
#[get("/")]
pub fn list(resolver: State<Resolver>) -> Result<Json<Challenge>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
