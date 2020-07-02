#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

extern crate chrono;

mod controllers;
mod db;
mod models;
mod schema;

use crate::models::ApiResult;

use rocket::Request;
use rocket_contrib::json::Json;

#[catch(422)]
fn unprocessable_entity(_: &Request) -> Json<ApiResult> {
    Json(ApiResult {
        code: 422,
        message: "Unable to parse the request".to_string(),
    })
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .register(catchers![unprocessable_entity])
        .attach(db::DbConn::fairing())
        .mount(
            "/",
            routes![
                index,
                controllers::users::get_user,
                controllers::users::create_user,
            ],
        )
        .launch();
}
