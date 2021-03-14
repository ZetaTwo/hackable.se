/*! `/challenges` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{err_msg, Error},
    domain::{challenges::Challenge, challenges::ChallengeId, Resolver},
};

/** `GET /challenges/<id>` */
#[get("/<id>")]
pub fn get(id: ChallengeId, resolver: State<Resolver>) -> Result<Json<Challenge>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `POST /challenges/<id>/instances` */
#[post("/<id>/instances", data = "<_instance_request>", format = "json")]
pub fn start_challenge(
    id: ChallengeId,
    resolver: State<Resolver>,
    _instance_request: String,
) -> Result<Json<Challenge>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `POST /challenges/<id>/solves` */
#[post("/<id>/solves", data = "<_solve_request>", format = "json")]
pub fn solve(
    id: ChallengeId,
    resolver: State<Resolver>,
    _solve_request: String,
) -> Result<Json<Challenge>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}

/** `GET /challenges` */
#[get("/")]
pub fn list(resolver: State<Resolver>) -> Result<Json<Challenge>, Error> {
    Err(Error::Other(err_msg("not implemented")))
}
