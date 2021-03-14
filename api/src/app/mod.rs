/*!
Rocket app configuration.
*/

use crate::{db, domain::Resolver};
use rocket;
use rocket_contrib::json::Json;

mod error;
mod id;

pub mod challenges;
pub mod sessions;
pub mod tags;
pub mod users;

#[catch(422)]
fn unprocessable_entity(_: &rocket::Request) -> Json<ApiResult> {
    Json(ApiResult {
        code: 422,
        message: "Unable to parse the request".to_string(),
    })
}

pub fn manage_database_resolver(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    let conn = db::DbConn::get_one(&rocket).expect("database connection");
    rocket.manage(Resolver::database_resolver(conn))
}

pub fn start() {
    info!("starting up");

    rocket::ignite()
        .register(catchers![unprocessable_entity])
        .attach(rocket::fairing::AdHoc::on_attach(
            "Argon2 secret key",
            |rocket| match rocket.config().get_string("argon_secret_key") {
                Err(err) => {
                    error!("Failed to read Argon2 secret key from config");
                    Err(rocket)
                }
                Ok(argon_secret_key) => {
                    Ok(rocket.manage(PasswordHashingConfig::new(argon_secret_key)))
                }
            },
        ))
        .attach(db::DbConn::fairing())
        .attach(rocket::fairing::AdHoc::on_attach(
            "Database Migrations",
            db::run_db_migrations,
        ))
        .attach(rocket::fairing::AdHoc::on_attach(
            "Resolver",
            manage_database_resolver,
        ))
        //.manage(Resolver::default())
        .mount(
            "/challenges",
            routes![
                challenges::list,
                challenges::get,
                challenges::start_challenge,
                challenges::solve
            ],
        )
        .mount("/tags", routes![tags::list, tags::get])
        .mount(
            "/users",
            routes![
                users::list,
                users::get,
                users::create,
                users::update,
                users::delete
            ],
        )
        .mount(
            "/sessions",
            routes![
                sessions::get,
                sessions::login,
                sessions::refresh,
                sessions::delete
            ],
        )
        .register(catchers![error::not_found, error::internal_error])
        .launch();
}
