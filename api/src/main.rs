#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

extern crate argonautica;
extern crate chrono;

mod controllers;
mod db;
mod models;
mod schema;
#[cfg(test)]
mod tests;

use crate::models::password_hash::PasswordHashingConfig;
use crate::models::ApiResult;

use rocket::fairing::AdHoc;
use rocket::Request;
use rocket_contrib::json::Json;

#[catch(422)]
fn unprocessable_entity(_: &Request) -> Json<ApiResult> {
    Json(ApiResult {
        code: 422,
        message: "Unable to parse the request".to_string(),
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .register(catchers![unprocessable_entity])
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_attach(
            "Database Migrations",
            db::run_db_migrations,
        ))
        .attach(AdHoc::on_attach("Argon2 secret key", |rocket| match rocket
            .config()
            .get_string("argon_secret_key")
        {
            Err(err) => {
                error!("Failed to read Argon2 secret key from config");
                Err(rocket)
            }
            Ok(argon_secret_key) => Ok(rocket.manage(PasswordHashingConfig::new(argon_secret_key))),
        }))
        .mount(
            "/",
            routes![
                controllers::users::get_user,
                controllers::users::create_user,
                controllers::users::update_user,
                controllers::challenges::list_challenges,
                controllers::challenges::get_challenge,
                controllers::challenges::solve_challenge,
                controllers::challenges::start_challenge,
                controllers::sessions::get_session,
                controllers::sessions::create_session,
                controllers::sessions::update_session,
                controllers::sessions::delete_session,
                controllers::tags::list_tags,
                controllers::tags::get_tag,
            ],
        )
}

fn main() {
    rocket().launch();
}
