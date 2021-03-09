/*!
Rocket app configuration.
*/

use crate::domain::Resolver;
use rocket;

mod error;
mod id;

pub mod users;
pub mod challenges;
pub mod sessions;
pub mod tags;

pub fn start() {
    info!("starting up");

    rocket::ignite()
        .manage(Resolver::default())
        .mount(
            "/challenges",
            routes![challenges::list, challenges::get],
        )
        .mount(
            "/tags",
            routes![tags::list, tags::get],
        )
        .mount(
            "/users",
            routes![users::get, users::create, users::update, users::delete]
        )
        .mount(
            "/sessions",
            routes![sessions::get, sessions::login, users::refresh, sessions::delete]
        )
        .register(catchers![error::not_found, error::internal_error])
        .launch();
}
