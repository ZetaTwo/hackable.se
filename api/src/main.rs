#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod db;
mod users;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
       .attach(db::DbConn::fairing())
       .mount("/", routes![
           index,
           users::get_user,
           users::create_user,
        ])
       .launch();
}
